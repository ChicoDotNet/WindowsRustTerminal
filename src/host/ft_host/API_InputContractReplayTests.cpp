// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#include "precomp.h"

#include <future>

using namespace WEX::TestExecution;

namespace
{
    // Contract Replay provenance:
    // dev/miniksa/input2 @ eb72c7fd3b494107f4c7ff7a78de219ddfbe8949
    // src/host/ft_host/API_InputTests.cpp::TestCookedTextEntry
    std::vector<INPUT_RECORD> _stringToInputs(const std::wstring_view text)
    {
        std::vector<INPUT_RECORD> result;
        result.reserve(text.size() * 2);

        for (const auto wch : text)
        {
            INPUT_RECORD record{};
            record.EventType = KEY_EVENT;
            record.Event.KeyEvent.bKeyDown = TRUE;
            record.Event.KeyEvent.dwControlKeyState = 0;
            record.Event.KeyEvent.uChar.UnicodeChar = wch;
            record.Event.KeyEvent.wRepeatCount = 1;
            record.Event.KeyEvent.wVirtualKeyCode = VkKeyScanW(wch);
            record.Event.KeyEvent.wVirtualScanCode = gsl::narrow<WORD>(MapVirtualKeyW(record.Event.KeyEvent.wVirtualKeyCode, MAPVK_VK_TO_VSC));

            result.emplace_back(record);

            record.Event.KeyEvent.bKeyDown = FALSE;
            result.emplace_back(record);
        }

        return result;
    }

    HRESULT _sendStringToInput(const HANDLE input, const std::wstring_view text)
    {
        const auto records = _stringToInputs(text);
        DWORD written = 0;
        RETURN_IF_WIN32_BOOL_FALSE(WriteConsoleInputW(input, records.data(), gsl::narrow<DWORD>(records.size()), &written));
        RETURN_HR_IF(E_UNEXPECTED, written != records.size());
        return S_OK;
    }

    HRESULT _readConsoleAWithTimeout(const HANDLE input, std::string& buffer, const bool async = true)
    {
        if (async)
        {
            auto read = std::async(std::launch::async, [&] {
                return _readConsoleAWithTimeout(input, buffer, false);
            });

            if (read.wait_for(std::chrono::seconds{ 5 }) != std::future_status::ready)
            {
                // Unblock the reader before returning the timeout so the async task can finish cleanly.
                RETURN_IF_FAILED(_sendStringToInput(input, L"a\r\n"));
                RETURN_NTSTATUS(STATUS_TIMEOUT);
            }

            return read.get();
        }

        DWORD read = 0;
        RETURN_IF_WIN32_BOOL_FALSE(ReadConsoleA(input, buffer.data(), gsl::narrow<DWORD>(buffer.size()), &read, nullptr));
        buffer.resize(read);
        return S_OK;
    }
}

class InputContractReplayTests
{
    BEGIN_TEST_CLASS(InputContractReplayTests)
    END_TEST_CLASS()

    BEGIN_TEST_METHOD(TestCookedTextEntryContractReplay)
        TEST_METHOD_PROPERTY(L"TestTimeout", L"00:00:15")
        TEST_METHOD_PROPERTY(L"IsolationLevel", L"Method")
    END_TEST_METHOD()
};

void InputContractReplayTests::TestCookedTextEntryContractReplay()
{
    const auto input = GetStdInputHandle();
    VERIFY_IS_NOT_NULL(input);

    DWORD originalMode = 0;
    VERIFY_WIN32_BOOL_SUCCEEDED(GetConsoleMode(input, &originalMode));

    auto restoreInput = wil::scope_exit([&] {
        FlushConsoleInputBuffer(input);
        SetConsoleMode(input, originalMode);
    });

    VERIFY_WIN32_BOOL_SUCCEEDED(FlushConsoleInputBuffer(input));

    constexpr DWORD cookedMode = ENABLE_ECHO_INPUT | ENABLE_LINE_INPUT | ENABLE_PROCESSED_INPUT;
    VERIFY_WIN32_BOOL_SUCCEEDED(SetConsoleMode(input, cookedMode));

    VERIFY_SUCCEEDED(_sendStringToInput(input, L"foo\r\n"));

    std::string actual(500, '\0');
    VERIFY_SUCCEEDED(_readConsoleAWithTimeout(input, actual));

    const std::string expected{ "foo\r\n" };
    VERIFY_ARE_EQUAL(expected, actual);
}
