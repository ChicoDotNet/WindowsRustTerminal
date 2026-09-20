// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT license.

#pragma once

#include "IStateMachineEngine.hpp"
#include "../../../rust/terminal-parser-ffi/include/terminal_parser_ffi.h"

#include <limits>
#include <span>
#include <utility>
#include <vector>

namespace Microsoft::Console::VirtualTerminal
{
    // Narrow adapter from the Rust parser ABI back into the existing native engine.
    // It intentionally owns no parsing semantics. Temporary vectors only materialize
    // borrowed FFI parameter views for the duration of an engine dispatch.
    class RustStateMachineBridge final
    {
    public:
        explicit RustStateMachineBridge(IStateMachineEngine& engine) noexcept :
            _engine{ engine }
        {
        }

        RustStateMachineBridge(const RustStateMachineBridge&) = delete;
        RustStateMachineBridge& operator=(const RustStateMachineBridge&) = delete;
        RustStateMachineBridge(RustStateMachineBridge&&) = delete;
        RustStateMachineBridge& operator=(RustStateMachineBridge&&) = delete;

        ~RustStateMachineBridge() noexcept
        {
            if (_handle != nullptr)
            {
                terminal_parser_ffi_state_machine_destroy(_handle);
            }
        }

        terminal_parser_ffi_status Initialize() noexcept
        {
            if (_handle != nullptr)
            {
                return TERMINAL_PARSER_FFI_OK;
            }

            terminal_parser_ffi_state_machine_callbacks callbacks{};
            callbacks.user_data = this;
            callbacks.execute = &ExecuteCallback;
            callbacks.print = &PrintCallback;
            callbacks.print_string = &PrintStringCallback;
            callbacks.esc = &EscCallback;
            callbacks.osc = &OscCallback;
            callbacks.dcs_dispatch = &DcsDispatchCallback;
            callbacks.dcs_put = &DcsPutCallback;

            auto status = terminal_parser_ffi_state_machine_create(&callbacks, &_handle);
            if (status == TERMINAL_PARSER_FFI_OK)
            {
                status = terminal_parser_ffi_state_machine_set_pass_through_callback(_handle, &PassThroughCallback);
            }
            if (status == TERMINAL_PARSER_FFI_OK)
            {
                status = terminal_parser_ffi_state_machine_set_execute_from_escape_callback(_handle, &ExecuteFromEscapeCallback);
            }
            if (status == TERMINAL_PARSER_FFI_OK)
            {
                status = terminal_parser_ffi_state_machine_set_vt52_esc_callback(_handle, &Vt52EscCallback);
            }
            if (status == TERMINAL_PARSER_FFI_OK)
            {
                status = terminal_parser_ffi_state_machine_set_ss3_callback(_handle, &Ss3Callback);
            }
            if (status == TERMINAL_PARSER_FFI_OK)
            {
                status = terminal_parser_ffi_state_machine_set_csi_lossless_callback(_handle, &CsiLosslessCallback);
            }
            if (status != TERMINAL_PARSER_FFI_OK)
            {
                if (_handle != nullptr)
                {
                    terminal_parser_ffi_state_machine_destroy(_handle);
                    _handle = nullptr;
                }
            }
            return status;
        }

        terminal_parser_ffi_status Process(const std::wstring_view text) noexcept
        {
            if (_handle == nullptr)
            {
                return TERMINAL_PARSER_FFI_INVALID_ARGUMENT;
            }
            static_assert(sizeof(wchar_t) == sizeof(uint16_t));
            return terminal_parser_ffi_state_machine_process_utf16(
                _handle,
                reinterpret_cast<const uint16_t*>(text.data()),
                text.size());
        }

        terminal_parser_ffi_status Reset() noexcept
        {
            if (_handle == nullptr)
            {
                return TERMINAL_PARSER_FFI_INVALID_ARGUMENT;
            }
            _dcsHandler = {};
            return terminal_parser_ffi_state_machine_reset_state(_handle);
        }

    private:
        static RustStateMachineBridge* _Self(void* userData) noexcept
        {
            return static_cast<RustStateMachineBridge*>(userData);
        }

        static std::wstring_view _StringView(const uint16_t* text, const size_t textLen) noexcept
        {
            static_assert(sizeof(wchar_t) == sizeof(uint16_t));
            return { reinterpret_cast<const wchar_t*>(text), textLen };
        }

        static bool ExecuteCallback(void* userData, const uint16_t codeUnit) noexcept
        {
            try
            {
                return userData != nullptr && _Self(userData)->_engine.ActionExecute(static_cast<wchar_t>(codeUnit));
            }
            catch (...)
            {
                return false;
            }
        }

        static bool ExecuteFromEscapeCallback(void* userData, const uint16_t codeUnit) noexcept
        {
            try
            {
                return userData != nullptr && _Self(userData)->_engine.ActionExecuteFromEscape(static_cast<wchar_t>(codeUnit));
            }
            catch (...)
            {
                return false;
            }
        }

        static bool PrintCallback(void* userData, const uint16_t codeUnit) noexcept
        {
            try
            {
                return userData != nullptr && _Self(userData)->_engine.ActionPrint(static_cast<wchar_t>(codeUnit));
            }
            catch (...)
            {
                return false;
            }
        }

        static bool PrintStringCallback(void* userData, const uint16_t* text, const size_t textLen) noexcept
        {
            if (userData == nullptr || (textLen != 0 && text == nullptr))
            {
                return false;
            }
            try
            {
                return _Self(userData)->_engine.ActionPrintString(_StringView(text, textLen));
            }
            catch (...)
            {
                return false;
            }
        }

        static bool PassThroughCallback(void* userData, const uint16_t* text, const size_t textLen) noexcept
        {
            if (userData == nullptr || (textLen != 0 && text == nullptr))
            {
                return false;
            }
            try
            {
                return _Self(userData)->_engine.ActionPassThroughString(_StringView(text, textLen));
            }
            catch (...)
            {
                return false;
            }
        }

        static bool EscCallback(void* userData, const uint64_t id) noexcept
        {
            try
            {
                return userData != nullptr && _Self(userData)->_engine.ActionEscDispatch(VTID{ id });
            }
            catch (...)
            {
                return false;
            }
        }

        static bool Vt52EscCallback(void* userData,
                                    const uint64_t id,
                                    const int32_t* values,
                                    const uint8_t* present,
                                    const size_t parameterCount) noexcept
        {
            try
            {
                return userData != nullptr && _Self(userData)->_DispatchFlat(
                    id, values, present, parameterCount, &IStateMachineEngine::ActionVt52EscDispatch);
            }
            catch (...)
            {
                return false;
            }
        }

        static bool Ss3Callback(void* userData,
                                const uint16_t codeUnit,
                                const int32_t* values,
                                const uint8_t* present,
                                const size_t parameterCount) noexcept
        {
            if (userData == nullptr)
            {
                return false;
            }
            try
            {
                auto* self = _Self(userData);
                if (!self->_MaterializeFlat(values, present, parameterCount))
                {
                    return false;
                }
                const VTParameters parameters{ self->_parameters.data(), self->_parameters.size() };
                return self->_engine.ActionSs3Dispatch(static_cast<wchar_t>(codeUnit), parameters);
            }
            catch (...)
            {
                return false;
            }
        }

        static bool CsiLosslessCallback(void* userData,
                                        const uint64_t id,
                                        const int32_t* values,
                                        const uint8_t* present,
                                        const size_t parameterCount,
                                        const int32_t* subValues,
                                        const uint8_t* subPresent,
                                        const size_t subParameterCount,
                                        const size_t* subOffsets,
                                        const size_t* subCounts) noexcept
        {
            if (userData == nullptr)
            {
                return false;
            }
            try
            {
                return _Self(userData)->_DispatchCsi(
                    id,
                    values,
                    present,
                    parameterCount,
                    subValues,
                    subPresent,
                    subParameterCount,
                    subOffsets,
                    subCounts);
            }
            catch (...)
            {
                return false;
            }
        }

        static bool OscCallback(void* userData,
                                const int32_t parameter,
                                const uint16_t* text,
                                const size_t textLen) noexcept
        {
            if (userData == nullptr || parameter < 0 || (textLen != 0 && text == nullptr))
            {
                return false;
            }
            try
            {
                return _Self(userData)->_engine.ActionOscDispatch(static_cast<size_t>(parameter), _StringView(text, textLen));
            }
            catch (...)
            {
                return false;
            }
        }

        static bool DcsDispatchCallback(void* userData,
                                        const uint64_t id,
                                        const int32_t* values,
                                        const uint8_t* present,
                                        const size_t parameterCount) noexcept
        {
            if (userData == nullptr)
            {
                return false;
            }
            try
            {
                auto* self = _Self(userData);
                if (!self->_MaterializeFlat(values, present, parameterCount))
                {
                    return false;
                }
                const VTParameters parameters{ self->_parameters.data(), self->_parameters.size() };
                self->_dcsHandler = self->_engine.ActionDcsDispatch(VTID{ id }, parameters);
                return static_cast<bool>(self->_dcsHandler);
            }
            catch (...)
            {
                return false;
            }
        }

        static bool DcsPutCallback(void* userData, const uint16_t codeUnit) noexcept
        {
            if (userData == nullptr)
            {
                return false;
            }
            try
            {
                auto* self = _Self(userData);
                return self->_dcsHandler && self->_dcsHandler(static_cast<wchar_t>(codeUnit));
            }
            catch (...)
            {
                return false;
            }
        }

        bool _MaterializeFlat(const int32_t* values, const uint8_t* present, const size_t parameterCount)
        {
            if (parameterCount != 0 && (values == nullptr || present == nullptr))
            {
                return false;
            }
            _parameters.clear();
            _parameters.reserve(parameterCount);
            for (size_t i = 0; i < parameterCount; ++i)
            {
                _parameters.emplace_back(present[i] != 0 ? values[i] : -1);
            }
            return true;
        }

        bool _DispatchFlat(const uint64_t id,
                           const int32_t* values,
                           const uint8_t* present,
                           const size_t parameterCount,
                           bool (IStateMachineEngine::*dispatch)(VTID, VTParameters))
        {
            if (!_MaterializeFlat(values, present, parameterCount))
            {
                return false;
            }
            const VTParameters parameters{ _parameters.data(), _parameters.size() };
            return (_engine.*dispatch)(VTID{ id }, parameters);
        }

        bool _DispatchCsi(const uint64_t id,
                          const int32_t* values,
                          const uint8_t* present,
                          const size_t parameterCount,
                          const int32_t* subValues,
                          const uint8_t* subPresent,
                          const size_t subParameterCount,
                          const size_t* subOffsets,
                          const size_t* subCounts)
        {
            if ((parameterCount != 0 && (values == nullptr || present == nullptr || subOffsets == nullptr || subCounts == nullptr)) ||
                (subParameterCount != 0 && (subValues == nullptr || subPresent == nullptr)))
            {
                return false;
            }

            _parameters.clear();
            _subParameters.clear();
            _subParameterRanges.clear();
            _parameters.reserve(parameterCount);
            _subParameters.reserve(subParameterCount);
            _subParameterRanges.reserve(parameterCount);

            for (size_t i = 0; i < parameterCount; ++i)
            {
                _parameters.emplace_back(present[i] != 0 ? values[i] : -1);

                const auto offset = subOffsets[i];
                const auto count = subCounts[i];
                if (offset > subParameterCount || count > subParameterCount - offset)
                {
                    return false;
                }

                const auto end = offset + count;
                if (offset > std::numeric_limits<BYTE>::max() || end > std::numeric_limits<BYTE>::max())
                {
                    return false;
                }

                _subParameterRanges.emplace_back(static_cast<BYTE>(offset), static_cast<BYTE>(end));
            }

            for (size_t i = 0; i < subParameterCount; ++i)
            {
                _subParameters.emplace_back(subPresent[i] != 0 ? subValues[i] : -1);
            }

            const VTParameters parameters{
                std::span<const VTParameter>{ _parameters },
                std::span<const VTParameter>{ _subParameters },
                std::span<const std::pair<BYTE, BYTE>>{ _subParameterRanges }
            };
            return _engine.ActionCsiDispatch(VTID{ id }, parameters);
        }

        IStateMachineEngine& _engine;
        terminal_parser_ffi_state_machine_handle* _handle{ nullptr };
        IStateMachineEngine::StringHandler _dcsHandler;
        std::vector<VTParameter> _parameters;
        std::vector<VTParameter> _subParameters;
        std::vector<std::pair<BYTE, BYTE>> _subParameterRanges;
    };
}
