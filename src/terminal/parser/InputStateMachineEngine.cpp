// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#include "precomp.h"

#include "stateMachine.hpp"
#include "InputStateMachineEngine.hpp"
#include "terminal_parser_ffi.h"

#include <til/atomic.h>

#include "../../inc/unicode.hpp"
#include "../../interactivity/inc/VtApiRedirection.hpp"

using namespace Microsoft::Console::VirtualTerminal;

InputStateMachineEngine::InputStateMachineEngine(std::unique_ptr<IInteractDispatch> pDispatch) :
    _pDispatch(std::move(pDispatch)),
    _doubleClickTime(std::chrono::milliseconds(GetDoubleClickTime()))
{
    THROW_HR_IF_NULL(E_INVALIDARG, _pDispatch.get());
}

void InputStateMachineEngine::CaptureNextCursorPositionReport() noexcept
{
    _captureNextCursorPositionReport.store(true, std::memory_order_relaxed);
}

til::enumset<DeviceAttribute, uint64_t> InputStateMachineEngine::WaitUntilDA1(DWORD timeout) noexcept
{
    uint64_t val = 0;

    // atomic_wait() returns false when the timeout expires.
    // Technically we should decrement the timeout with each iteration,
    // but I suspect infinite spurious wake-ups are a theoretical problem.
    for (;;)
    {
        val = _deviceAttributes.load(std::memory_order_relaxed);
        if (val)
        {
            break;
        }

        if (!til::atomic_wait(_deviceAttributes, val, timeout))
        {
            break;
        }
    }

    // VtIo first sends a DSR CPR and then a DA1 request.
    // If we encountered a DA1 response here, the DSR request is definitely done now.
    _captureNextCursorPositionReport.store(false, std::memory_order_relaxed);

    return til::enumset<DeviceAttribute, uint64_t>::from_bits(val);
}

void InputStateMachineEngine::UnknownSequence() noexcept
{
}

bool InputStateMachineEngine::EncounteredWin32InputModeSequence() const noexcept
{
    return _encounteredWin32InputModeSequence;
}

bool InputStateMachineEngine::ActionExecute(const wchar_t wch)
{
    return _DoControlCharacter(wch, false);
}

bool InputStateMachineEngine::_DoControlCharacter(const wchar_t wch, const bool writeAlt)
{
    if (wch == UNICODE_ETX && !writeAlt)
    {
        static constexpr auto keyDown = SynthesizeKeyEvent(true, 1, L'C', 0, UNICODE_ETX, LEFT_CTRL_PRESSED);
        static constexpr auto keyUp = SynthesizeKeyEvent(false, 1, L'C', 0, UNICODE_ETX, LEFT_CTRL_PRESSED);
        _pDispatch->WriteCtrlKey(keyDown);
        _pDispatch->WriteCtrlKey(keyUp);
    }
    else if (wch >= '\x0' && wch < '\x20')
    {
        auto actualChar = wch;
        auto writeCtrl = true;
        auto success = false;

        short vkey = 0;
        DWORD modifierState = 0;

        switch (wch)
        {
        case L'\b':
            actualChar = '\x7f';
            success = _GenerateKeyFromChar(actualChar, vkey, modifierState);
            modifierState = 0;
            break;
        case L'\r':
            writeCtrl = false;
            success = _GenerateKeyFromChar(wch, vkey, modifierState);
            modifierState = 0;
            break;
        case L'\x1b':
            vkey = VK_ESCAPE;
            writeCtrl = false;
            success = true;
            break;
        case L'\t':
            writeCtrl = false;
            success = _GenerateKeyFromChar(actualChar, vkey, modifierState);
            break;
        default:
            success = _GenerateKeyFromChar(actualChar, vkey, modifierState);
            break;
        }

        if (success)
        {
            if (writeCtrl)
            {
                WI_SetFlag(modifierState, LEFT_CTRL_PRESSED);
            }
            if (writeAlt)
            {
                WI_SetFlag(modifierState, LEFT_ALT_PRESSED);
            }

            _WriteSingleKey(actualChar, vkey, modifierState);
        }
    }
    else if (wch == '\x7f')
    {
        _WriteSingleKey('\x8', VK_BACK, writeAlt ? LEFT_ALT_PRESSED : 0);
    }
    else
    {
        ActionPrint(wch);
    }
    return true;
}

bool InputStateMachineEngine::ActionExecuteFromEscape(const wchar_t wch)
{
    if (_pDispatch->IsVtInputEnabled())
    {
        return false;
    }

    return _DoControlCharacter(wch, true);
}

bool InputStateMachineEngine::ActionPrint(const wchar_t wch)
{
    short vkey = 0;
    DWORD modifierState = 0;
    if (_GenerateKeyFromChar(wch, vkey, modifierState))
    {
        _WriteSingleKey(wch, vkey, modifierState);
    }
    return true;
}

bool InputStateMachineEngine::ActionPrintString(const std::wstring_view string)
{
    if (!string.empty())
    {
        _pDispatch->WriteString(string);
    }
    return true;
}

bool InputStateMachineEngine::ActionPassThroughString(const std::wstring_view string)
{
    if (!string.empty())
    {
        _pDispatch->WriteStringRaw(string);
    }
    return true;
}

bool InputStateMachineEngine::ActionEscDispatch(const VTID id)
{
    if (_expectingStringTerminator && id == VTID("\\"))
    {
        _expectingStringTerminator = false;
        return false;
    }

    if (_pDispatch->IsVtInputEnabled())
    {
        return false;
    }

    const auto wch = gsl::narrow_cast<wchar_t>(id);

    if (wch == 0x7f)
    {
        _DoControlCharacter(wch, true);
    }
    else
    {
        DWORD modifierState = 0;
        short vk = 0;
        if (_GenerateKeyFromChar(wch, vk, modifierState))
        {
            modifierState = WI_SetFlag(modifierState, LEFT_ALT_PRESSED);
            _WriteSingleKey(wch, vk, modifierState);
        }
    }

    return true;
}

bool InputStateMachineEngine::ActionVt52EscDispatch(const VTID /*id*/, const VTParameters /*parameters*/) noexcept
{
    return false;
}

bool InputStateMachineEngine::ActionCsiDispatch(const VTID id, const VTParameters parameters)
{
    const auto vtInputEnabled = _pDispatch->IsVtInputEnabled();

    switch (id)
    {
    case CsiActionCodes::MouseDown:
    case CsiActionCodes::MouseUp:
    {
        if (vtInputEnabled)
        {
            return false;
        }

        DWORD buttonState = 0;
        DWORD eventFlags = 0;
        const auto firstParameter = parameters.at(0).value_or(0);
        const til::point uiPos{ parameters.at(1) - 1, parameters.at(2) - 1 };

        if (_UpdateSGRMouseButtonState(id, firstParameter, buttonState, eventFlags, uiPos))
        {
            const auto modifierState = _GetSGRMouseModifierState(firstParameter);
            _WriteMouseEvent(uiPos, buttonState, modifierState, eventFlags);
        }
        return true;
    }
    case CsiActionCodes::CSI_F3:
        if (_captureNextCursorPositionReport.exchange(false, std::memory_order_relaxed))
        {
            _pDispatch->MoveCursor(parameters.at(0), parameters.at(1));
            return true;
        }
        if (_encounteredWin32InputModeSequence)
        {
            return false;
        }
        if (vtInputEnabled)
        {
            return false;
        }
        [[fallthrough]];
    case CsiActionCodes::ArrowUp:
    case CsiActionCodes::ArrowDown:
    case CsiActionCodes::ArrowRight:
    case CsiActionCodes::ArrowLeft:
    case CsiActionCodes::Home:
    case CsiActionCodes::End:
    case CsiActionCodes::CSI_F1:
    case CsiActionCodes::CSI_F2:
    case CsiActionCodes::CSI_F4:
    {
        if (vtInputEnabled)
        {
            return false;
        }
        short vkey = 0;
        if (_GetCursorKeysVkey(id, vkey))
        {
            const auto modifierState = _GetCursorKeysModifierState(parameters, id);
            _WriteSingleKey(vkey, modifierState);
        }
        return true;
    }
    case CsiActionCodes::Generic:
    {
        if (vtInputEnabled)
        {
            return false;
        }
        short vkey = 0;
        if (_GetGenericVkey(parameters.at(0), vkey))
        {
            const auto modifierState = _GetGenericKeysModifierState(parameters);
            _WriteSingleKey(vkey, modifierState);
        }
        return true;
    }
    case CsiActionCodes::CursorBackTab:
        if (vtInputEnabled)
        {
            return false;
        }
        _WriteSingleKey(VK_TAB, SHIFT_PRESSED);
        return true;
    case CsiActionCodes::FocusIn:
        _pDispatch->FocusChanged(true);
        return true;
    case CsiActionCodes::FocusOut:
        _pDispatch->FocusChanged(false);
        return true;
    case CsiActionCodes::DA_DeviceAttributes:
        if (_deviceAttributes.load(std::memory_order_relaxed) == 0)
        {
            til::enumset<DeviceAttribute, uint64_t> attributes{ DeviceAttribute::__some__ };
            const auto len = parameters.size();
            if (len >= 2 && parameters.at(0).value() >= 61)
            {
                for (size_t i = 1; i < len; i++)
                {
                    const auto value = parameters.at(i).value();
                    if (value > 0 && value < 64)
                    {
                        attributes.set(static_cast<DeviceAttribute>(value));
                    }
                }
            }

            _deviceAttributes.fetch_or(attributes.bits(), std::memory_order_relaxed);
            til::atomic_notify_all(_deviceAttributes);
            return true;
        }
        return false;
    case CsiActionCodes::Win32KeyboardInput:
    {
        const auto key = _GenerateWin32Key(parameters);
        _pDispatch->WriteCtrlKey(key);
        _encounteredWin32InputModeSequence = true;
        return true;
    }
    default:
        return false;
    }
}

IStateMachineEngine::StringHandler InputStateMachineEngine::ActionDcsDispatch(const VTID /*id*/, const VTParameters /*parameters*/) noexcept
{
    _expectingStringTerminator = true;
    return nullptr;
}

bool InputStateMachineEngine::ActionSs3Dispatch(const wchar_t wch, const VTParameters /*parameters*/)
{
    if (_pDispatch->IsVtInputEnabled())
    {
        return false;
    }

    short vkey = 0;
    if (_GetSs3KeysVkey(wch, vkey))
    {
        _WriteSingleKey(vkey, 0);
    }

    return true;
}

bool InputStateMachineEngine::ActionOscDispatch(const size_t /*parameter*/, const std::wstring_view /*string*/) noexcept
{
    return false;
}

void InputStateMachineEngine::_GenerateWrappedSequence(const wchar_t wch,
                                                       const short vkey,
                                                       const DWORD modifierState,
                                                       InputEventQueue& input)
{
    input.reserve(input.size() + 8);

    const auto shift = WI_IsFlagSet(modifierState, SHIFT_PRESSED);
    const auto ctrl = WI_IsFlagSet(modifierState, LEFT_CTRL_PRESSED);
    const auto alt = WI_IsFlagSet(modifierState, LEFT_ALT_PRESSED);

    auto next = SynthesizeKeyEvent(true, 1, 0, 0, 0, 0);
    DWORD currentModifiers = 0;

    if (shift)
    {
        WI_SetFlag(currentModifiers, SHIFT_PRESSED);
        next.Event.KeyEvent.dwControlKeyState = currentModifiers;
        next.Event.KeyEvent.wVirtualKeyCode = VK_SHIFT;
        next.Event.KeyEvent.wVirtualScanCode = gsl::narrow_cast<WORD>(OneCoreSafeMapVirtualKeyW(VK_SHIFT, MAPVK_VK_TO_VSC));
        input.push_back(next);
    }
    if (alt)
    {
        WI_SetFlag(currentModifiers, LEFT_ALT_PRESSED);
        next.Event.KeyEvent.dwControlKeyState = currentModifiers;
        next.Event.KeyEvent.wVirtualKeyCode = VK_MENU;
        next.Event.KeyEvent.wVirtualScanCode = gsl::narrow_cast<WORD>(OneCoreSafeMapVirtualKeyW(VK_MENU, MAPVK_VK_TO_VSC));
        input.push_back(next);
    }
    if (ctrl)
    {
        WI_SetFlag(currentModifiers, LEFT_CTRL_PRESSED);
        next.Event.KeyEvent.dwControlKeyState = currentModifiers;
        next.Event.KeyEvent.wVirtualKeyCode = VK_CONTROL;
        next.Event.KeyEvent.wVirtualScanCode = gsl::narrow_cast<WORD>(OneCoreSafeMapVirtualKeyW(VK_CONTROL, MAPVK_VK_TO_VSC));
        input.push_back(next);
    }

    _GetSingleKeypress(wch, vkey, modifierState, input);

    next.Event.KeyEvent.bKeyDown = FALSE;

    if (ctrl)
    {
        WI_ClearFlag(currentModifiers, LEFT_CTRL_PRESSED);
        next.Event.KeyEvent.dwControlKeyState = currentModifiers;
        next.Event.KeyEvent.wVirtualKeyCode = VK_CONTROL;
        next.Event.KeyEvent.wVirtualScanCode = gsl::narrow_cast<WORD>(OneCoreSafeMapVirtualKeyW(VK_CONTROL, MAPVK_VK_TO_VSC));
        input.push_back(next);
    }
    if (alt)
    {
        WI_ClearFlag(currentModifiers, LEFT_ALT_PRESSED);
        next.Event.KeyEvent.dwControlKeyState = currentModifiers;
        next.Event.KeyEvent.wVirtualKeyCode = VK_MENU;
        next.Event.KeyEvent.wVirtualScanCode = gsl::narrow_cast<WORD>(OneCoreSafeMapVirtualKeyW(VK_MENU, MAPVK_VK_TO_VSC));
        input.push_back(next);
    }
    if (shift)
    {
        WI_ClearFlag(currentModifiers, SHIFT_PRESSED);
        next.Event.KeyEvent.dwControlKeyState = currentModifiers;
        next.Event.KeyEvent.wVirtualKeyCode = VK_SHIFT;
        next.Event.KeyEvent.wVirtualScanCode = gsl::narrow_cast<WORD>(OneCoreSafeMapVirtualKeyW(VK_SHIFT, MAPVK_VK_TO_VSC));
        input.push_back(next);
    }
}

void InputStateMachineEngine::_GetSingleKeypress(const wchar_t wch,
                                                 const short vkey,
                                                 const DWORD modifierState,
                                                 InputEventQueue& input)
{
    input.reserve(input.size() + 2);

    const auto sc = gsl::narrow_cast<WORD>(OneCoreSafeMapVirtualKeyW(vkey, MAPVK_VK_TO_VSC));
    auto rec = SynthesizeKeyEvent(true, 1, vkey, sc, wch, modifierState);

    input.push_back(rec);
    rec.Event.KeyEvent.bKeyDown = FALSE;
    input.push_back(rec);
}

void InputStateMachineEngine::_WriteSingleKey(const wchar_t wch, const short vkey, const DWORD modifierState)
{
    InputEventQueue inputEvents;
    _GenerateWrappedSequence(wch, vkey, modifierState, inputEvents);
    _pDispatch->WriteInput(inputEvents);
}

void InputStateMachineEngine::_WriteSingleKey(const short vkey, const DWORD modifierState)
{
    const auto wch = gsl::narrow_cast<wchar_t>(OneCoreSafeMapVirtualKeyW(vkey, MAPVK_VK_TO_CHAR));
    _WriteSingleKey(wch, vkey, modifierState);
}

void InputStateMachineEngine::_WriteMouseEvent(const til::point uiPos, const DWORD buttonState, const DWORD controlKeyState, const DWORD eventFlags)
{
    const auto rgInput = SynthesizeMouseEvent(uiPos, buttonState, controlKeyState, eventFlags);
    _pDispatch->WriteInput({ &rgInput, 1 });
}

DWORD InputStateMachineEngine::_GetCursorKeysModifierState(const VTParameters parameters, const VTID id) noexcept
{
    return terminal_parser_ffi_input_cursor_modifier_state(
        gsl::narrow_cast<uint16_t>(id),
        gsl::narrow_cast<uint32_t>(parameters.at(1)));
}

DWORD InputStateMachineEngine::_GetGenericKeysModifierState(const VTParameters parameters) noexcept
{
    return terminal_parser_ffi_input_generic_modifier_state(
        static_cast<int32_t>(parameters.at(0)),
        gsl::narrow_cast<uint32_t>(parameters.at(1)));
}

DWORD InputStateMachineEngine::_GetSGRMouseModifierState(const size_t modifierParam) noexcept
{
    return terminal_parser_ffi_input_sgr_mouse_modifier_state(gsl::narrow_cast<uint32_t>(modifierParam));
}

DWORD InputStateMachineEngine::_GetModifier(const size_t modifierParam) noexcept
{
    return terminal_parser_ffi_input_vt_modifier_state(gsl::narrow_cast<uint32_t>(modifierParam));
}

bool InputStateMachineEngine::_UpdateSGRMouseButtonState(const VTID id,
                                                         const size_t sgrEncoding,
                                                         DWORD& buttonState,
                                                         DWORD& eventFlags,
                                                         const til::point uiPos)
{
    buttonState = _mouseButtonState;
    eventFlags = 0;

    const auto buttonID = (sgrEncoding & 0x3) | ((sgrEncoding & 0xC0) >> 4);
    const auto currentTime = std::chrono::steady_clock::now();
    DWORD buttonFlag = 0;
    switch (buttonID)
    {
    case CsiMouseButtonCodes::Left:
        buttonFlag = FROM_LEFT_1ST_BUTTON_PRESSED;
        break;
    case CsiMouseButtonCodes::Right:
        buttonFlag = RIGHTMOST_BUTTON_PRESSED;
        break;
    case CsiMouseButtonCodes::Middle:
        buttonFlag = FROM_LEFT_2ND_BUTTON_PRESSED;
        break;
    case CsiMouseButtonCodes::ScrollBack:
    {
        buttonState |= SCROLL_DELTA_BACKWARD;
        eventFlags |= MOUSE_WHEELED;
        break;
    }
    case CsiMouseButtonCodes::ScrollForward:
    {
        buttonState |= SCROLL_DELTA_FORWARD;
        eventFlags |= MOUSE_WHEELED;
        break;
    }
    case CsiMouseButtonCodes::ScrollLeft:
    {
        buttonState |= SCROLL_DELTA_BACKWARD;
        eventFlags |= MOUSE_HWHEELED;
        break;
    }
    case CsiMouseButtonCodes::ScrollRight:
    {
        buttonState |= SCROLL_DELTA_FORWARD;
        eventFlags |= MOUSE_HWHEELED;
        break;
    }
    case CsiMouseButtonCodes::Released:
        break;
    default:
        return false;
    }

    switch (id)
    {
    case CsiActionCodes::MouseDown:
        buttonState |= buttonFlag;
        if (_lastMouseClickPos && _lastMouseClickTime && _lastMouseClickButton &&
            uiPos == _lastMouseClickPos &&
            (currentTime - _lastMouseClickTime.value()) < _doubleClickTime &&
            buttonID == _lastMouseClickButton)
        {
            eventFlags |= DOUBLE_CLICK;
            _lastMouseClickPos.reset();
            _lastMouseClickTime.reset();
            _lastMouseClickButton.reset();
        }
        else if (buttonID == CsiMouseButtonCodes::Left ||
                 buttonID == CsiMouseButtonCodes::Right ||
                 buttonID == CsiMouseButtonCodes::Middle)
        {
            _lastMouseClickPos = uiPos;
            _lastMouseClickTime = currentTime;
            _lastMouseClickButton = buttonID;
        }
        break;
    case CsiActionCodes::MouseUp:
        buttonState &= (~buttonFlag);
        break;
    default:
        return false;
    }

    if (WI_IsFlagSet(sgrEncoding, CsiMouseModifierCodes::Drag))
    {
        eventFlags |= MOUSE_MOVED;
    }

    _mouseButtonState = LOWORD(buttonState);

    return true;
}

bool InputStateMachineEngine::_GetGenericVkey(const GenericKeyIdentifiers identifier, short& vkey) const
{
    const auto mapped = terminal_parser_ffi_input_generic_vkey(static_cast<int32_t>(identifier));
    vkey = gsl::narrow_cast<short>(mapped);
    return mapped != 0;
}

bool InputStateMachineEngine::_GetCursorKeysVkey(const VTID id, short& vkey) const
{
    const auto mapped = terminal_parser_ffi_input_cursor_vkey(gsl::narrow_cast<uint16_t>(id));
    vkey = gsl::narrow_cast<short>(mapped);
    return mapped != 0;
}

bool InputStateMachineEngine::_GetSs3KeysVkey(const wchar_t wch, short& vkey) const
{
    const auto mapped = terminal_parser_ffi_input_ss3_vkey(gsl::narrow_cast<uint16_t>(wch));
    vkey = gsl::narrow_cast<short>(mapped);
    return mapped != 0;
}

bool InputStateMachineEngine::_GenerateKeyFromChar(const wchar_t wch,
                                                   short& vkey,
                                                   DWORD& modifierState) noexcept
{
    const auto keyscan = OneCoreSafeVkKeyScanW(wch);

    short key = LOBYTE(keyscan);

    const short keyscanModifiers = HIBYTE(keyscan);

    if (key == -1 && keyscanModifiers == -1)
    {
        return false;
    }

    short modifierFlags = 0 |
                          (WI_IsFlagSet(keyscanModifiers, KEYSCAN_SHIFT) ? SHIFT_PRESSED : 0) |
                          (WI_IsFlagSet(keyscanModifiers, KEYSCAN_CTRL) ? LEFT_CTRL_PRESSED : 0) |
                          (WI_IsFlagSet(keyscanModifiers, KEYSCAN_ALT) ? LEFT_ALT_PRESSED : 0);

    vkey = key;
    modifierState = modifierFlags;

    return true;
}

bool InputStateMachineEngine::_GetWindowManipulationType(const std::span<const size_t> parameters,
                                                         unsigned int& function) const noexcept
{
    auto success = false;
    function = DispatchTypes::WindowManipulationType::Invalid;

    if (!parameters.empty())
    {
        switch (til::at(parameters, 0))
        {
        case DispatchTypes::WindowManipulationType::RefreshWindow:
            function = DispatchTypes::WindowManipulationType::RefreshWindow;
            success = true;
            break;
        case DispatchTypes::WindowManipulationType::ResizeWindowInCharacters:
            function = DispatchTypes::WindowManipulationType::ResizeWindowInCharacters;
            success = true;
            break;
        default:
            success = false;
        }
    }

    return success;
}

INPUT_RECORD InputStateMachineEngine::_GenerateWin32Key(const VTParameters& parameters)
{
    uint32_t presentMask = 0;
    int32_t values[6]{};
    for (size_t index = 0; index < 6; ++index)
    {
        const auto parameter = parameters.at(index);
        if (parameter.has_value())
        {
            presentMask |= 1u << index;
            values[index] = parameter.value();
        }
    }

    terminal_parser_ffi_key_event key{};
    const auto status = terminal_parser_ffi_input_win32_key_fields(
        presentMask,
        values[0],
        values[1],
        values[2],
        values[3],
        values[4],
        values[5],
        &key);
    THROW_HR_IF(E_UNEXPECTED, status != TERMINAL_PARSER_FFI_OK);

    return SynthesizeKeyEvent(
        key.key_down != 0,
        key.repeat_count,
        key.virtual_key,
        key.scan_code,
        static_cast<wchar_t>(key.unicode_char),
        key.control_key_state);
}
