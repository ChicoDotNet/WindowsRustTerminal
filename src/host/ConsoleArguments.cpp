// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#include "precomp.h"
#include "ConsoleArguments.hpp"
#include "../types/inc/utils.hpp"
#include "console_arguments_ffi.h"
#include <shellapi.h>
using namespace Microsoft::Console::Utils;

const std::wstring_view ConsoleArguments::HEADLESS_ARG = L"--headless";
const std::wstring_view ConsoleArguments::SERVER_HANDLE_ARG = L"--server";
const std::wstring_view ConsoleArguments::SIGNAL_HANDLE_ARG = L"--signal";
const std::wstring_view ConsoleArguments::HANDLE_PREFIX = L"0x";
const std::wstring_view ConsoleArguments::CLIENT_COMMANDLINE_ARG = L"--";
const std::wstring_view ConsoleArguments::FORCE_V1_ARG = L"-ForceV1";
const std::wstring_view ConsoleArguments::FORCE_NO_HANDOFF_ARG = L"-ForceNoHandoff";
const std::wstring_view ConsoleArguments::FILEPATH_LEADER_PREFIX = L"\\??\\";
const std::wstring_view ConsoleArguments::WIDTH_ARG = L"--width";
const std::wstring_view ConsoleArguments::HEIGHT_ARG = L"--height";
const std::wstring_view ConsoleArguments::INHERIT_CURSOR_ARG = L"--inheritcursor";
const std::wstring_view ConsoleArguments::FEATURE_ARG = L"--feature";
const std::wstring_view ConsoleArguments::FEATURE_PTY_ARG = L"pty";
const std::wstring_view ConsoleArguments::COM_SERVER_ARG = L"-Embedding";

ConsoleArguments::ConsoleArguments(const std::wstring& commandline,
                                   const HANDLE hStdIn,
                                   const HANDLE hStdOut) :
    _commandline(commandline),
    _vtInHandle(hStdIn),
    _vtOutHandle(hStdOut)
{
}

ConsoleArguments::ConsoleArguments() :
    ConsoleArguments(L"", nullptr, nullptr)
{
}

[[nodiscard]] HRESULT ConsoleArguments::ParseCommandline()
{
    if (_commandline.empty())
    {
        return S_OK;
    }

    auto copy = _commandline;
    auto argc = 0;
    wil::unique_hlocal_ptr<PWSTR[]> argv;
    argv.reset(CommandLineToArgvW(copy.c_str(), &argc));
    RETURN_LAST_ERROR_IF(argv == nullptr);

    std::vector<terminal_parser_ffi_console_argument_token> tokens;
    tokens.reserve(argc > 1 ? static_cast<size_t>(argc - 1) : 0);
    for (auto i = 1; i < argc; ++i)
    {
        const std::wstring_view token{ argv[i] };
        tokens.push_back({ reinterpret_cast<const uint16_t*>(token.data()), token.size() });
    }

    terminal_parser_ffi_console_argument_plan plan{};
    const auto tokenData = tokens.empty() ? nullptr : tokens.data();
    auto status = terminal_parser_ffi_console_arguments_plan(tokenData,
                                                             tokens.size(),
                                                             nullptr,
                                                             0,
                                                             nullptr,
                                                             0,
                                                             &plan);
    RETURN_HR_IF(E_INVALIDARG,
                 status != TERMINAL_PARSER_FFI_OK && status != TERMINAL_PARSER_FFI_BUFFER_TOO_SMALL);

    std::wstring clientCommandline(plan.client_commandline_len, L'\0');
    std::wstring textMeasurement(plan.text_measurement_len, L'\0');
    if (status == TERMINAL_PARSER_FFI_BUFFER_TOO_SMALL)
    {
        status = terminal_parser_ffi_console_arguments_plan(
            tokenData,
            tokens.size(),
            clientCommandline.empty() ? nullptr : reinterpret_cast<uint16_t*>(clientCommandline.data()),
            clientCommandline.size(),
            textMeasurement.empty() ? nullptr : reinterpret_cast<uint16_t*>(textMeasurement.data()),
            textMeasurement.size(),
            &plan);
        RETURN_HR_IF(E_INVALIDARG, status != TERMINAL_PARSER_FFI_OK);
    }

    _clientCommandline = std::move(clientCommandline);
    _textMeasurement = std::move(textMeasurement);
    _width = plan.width;
    _height = plan.height;
    _ambiguousIsWide = (plan.flags & TERMINAL_PARSER_FFI_CONSOLE_ARGUMENT_AMBIGUOUS_IS_WIDE) != 0;
    _forceV1 = (plan.flags & TERMINAL_PARSER_FFI_CONSOLE_ARGUMENT_FORCE_V1) != 0;
    _forceNoHandoff = (plan.flags & TERMINAL_PARSER_FFI_CONSOLE_ARGUMENT_FORCE_NO_HANDOFF) != 0;
    _headless = (plan.flags & TERMINAL_PARSER_FFI_CONSOLE_ARGUMENT_HEADLESS) != 0;
    _runAsComServer = (plan.flags & TERMINAL_PARSER_FFI_CONSOLE_ARGUMENT_RUN_AS_COM_SERVER) != 0;
    _createServerHandle = (plan.flags & TERMINAL_PARSER_FFI_CONSOLE_ARGUMENT_CREATE_SERVER_HANDLE) != 0;
    _inheritCursor = (plan.flags & TERMINAL_PARSER_FFI_CONSOLE_ARGUMENT_INHERIT_CURSOR) != 0;
    _serverHandle = plan.has_server_handle ? plan.server_handle : 0;
    _signalHandle = plan.has_signal_handle ? plan.signal_handle : 0;

    return S_OK;
}

bool ConsoleArguments::HasVtHandles() const
{
    return IsValidHandle(_vtInHandle) && IsValidHandle(_vtOutHandle);
}

bool ConsoleArguments::HasSignalHandle() const
{
    return IsValidHandle(GetSignalHandle());
}

bool ConsoleArguments::InConptyMode() const noexcept
{
    return IsValidHandle(_vtInHandle) || IsValidHandle(_vtOutHandle) || HasSignalHandle();
}

bool ConsoleArguments::IsHeadless() const
{
    return _headless;
}

bool ConsoleArguments::ShouldCreateServerHandle() const
{
    return _createServerHandle;
}

bool ConsoleArguments::ShouldRunAsComServer() const
{
    return _runAsComServer;
}

HANDLE ConsoleArguments::GetServerHandle() const
{
    return ULongToHandle(_serverHandle);
}

HANDLE ConsoleArguments::GetSignalHandle() const
{
    return ULongToHandle(_signalHandle);
}

HANDLE ConsoleArguments::GetVtInHandle() const
{
    return _vtInHandle;
}

HANDLE ConsoleArguments::GetVtOutHandle() const
{
    return _vtOutHandle;
}

std::wstring ConsoleArguments::GetOriginalCommandLine() const
{
    return _commandline;
}

std::wstring ConsoleArguments::GetClientCommandline() const
{
    return _clientCommandline;
}

const std::wstring& ConsoleArguments::GetTextMeasurement() const
{
    return _textMeasurement;
}

bool ConsoleArguments::GetAmbiguousIsWide() const
{
    return _ambiguousIsWide;
}

bool ConsoleArguments::GetForceV1() const
{
    return _forceV1;
}

bool ConsoleArguments::GetForceNoHandoff() const
{
    return _forceNoHandoff;
}

short ConsoleArguments::GetWidth() const
{
    return _width;
}

short ConsoleArguments::GetHeight() const
{
    return _height;
}

bool ConsoleArguments::GetInheritCursor() const
{
    return _inheritCursor;
}

#ifdef UNIT_TESTING
void ConsoleArguments::EnableConptyModeForTests()
{
    _headless = true;
    _vtInHandle = GetStdHandle(STD_INPUT_HANDLE);
    _vtOutHandle = GetStdHandle(STD_OUTPUT_HANDLE);
}
#endif
