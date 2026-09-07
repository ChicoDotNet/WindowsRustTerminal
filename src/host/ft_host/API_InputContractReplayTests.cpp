// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#include "precomp.h"

#include <functional>
#include <future>

using namespace WEX::TestExecution;

namespace
{
    // Contract Replay provenance:
    // dev/miniksa/input2 @ eb72c7fd3b494107f4c7ff7a78de219ddfbe8949
    // Replays the modern (CHv2) contract surface as one functional unit:
    // cooked text/aliases, ANSI code-page permutations, DBCS lead/trail handling,
    // and code-page changes across partial reads.
    constexpr std::array<std::wstring_view, 4> wide{
        L"\u03b1", // alpha
        L"\u03b2", // beta
        L"\u03b4", // delta
        L"\u03b5", // epsilon
    };

    constexpr std::array<std::string_view, 4> char437{
        "\xe0",
        "\xe1",
        "\xeb",
        "\xee",
    };

    constexpr std::array<std::string_view, 4> char932{
        "\x83\xbf",
        "\x83\xc0",
        "\x83\xc2",
        "\x83\xc3",
    };

    constexpr std::string_view crlf{ "\r\n" };

    enum class ReadMode
    {
        Cooked,
        Raw,
        Direct,
    };

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

    HRESULT _readConsoleInputAWithTimeout(const HANDLE input, std::string& buffer, const bool async = true)
    {
        if (async)
        {
            auto read = std::async(std::launch::async, [&] {
                return _readConsoleInputAWithTimeout(input, buffer, false);
            });

            if (read.wait_for(std::chrono::seconds{ 5 }) != std::future_status::ready)
            {
                RETURN_IF_FAILED(_sendStringToInput(input, L"a\r\n"));
                RETURN_NTSTATUS(STATUS_TIMEOUT);
            }

            return read.get();
        }

        const auto requested = buffer.size();
        buffer.clear();

        while (buffer.size() < requested)
        {
            std::vector<INPUT_RECORD> records(requested - buffer.size());
            DWORD read = 0;
            RETURN_IF_WIN32_BOOL_FALSE(ReadConsoleInputA(input, records.data(), gsl::narrow<DWORD>(records.size()), &read));

            for (DWORD i = 0; i < read; ++i)
            {
                const auto& record = records[i];
                if (record.EventType == KEY_EVENT && !record.Event.KeyEvent.bKeyDown)
                {
                    buffer.push_back(record.Event.KeyEvent.uChar.AsciiChar);
                }
            }
        }

        return S_OK;
    }

    HRESULT _readByMode(const HANDLE input, const ReadMode mode, std::string& buffer)
    {
        switch (mode)
        {
        case ReadMode::Cooked:
        case ReadMode::Raw:
            return _readConsoleAWithTimeout(input, buffer);
        case ReadMode::Direct:
            return _readConsoleInputAWithTimeout(input, buffer);
        }

        RETURN_HR(E_UNEXPECTED);
    }

    void _verifyRead(const HANDLE input, const ReadMode mode, const std::string_view expected)
    {
        std::string actual(expected.size(), '\0');
        VERIFY_SUCCEEDED(_readByMode(input, mode, actual));
        VERIFY_ARE_EQUAL(std::string{ expected }, actual);
    }

    void _forEachReadMode(const std::function<void(HANDLE, ReadMode)>& contract)
    {
        constexpr std::array modes{
            ReadMode::Cooked,
            ReadMode::Raw,
            ReadMode::Direct,
        };

        for (const auto mode : modes)
        {
            const auto input = GetStdInputHandle();
            VERIFY_IS_NOT_NULL(input);

            DWORD originalMode = 0;
            VERIFY_WIN32_BOOL_SUCCEEDED(GetConsoleMode(input, &originalMode));
            const auto originalCodePage = GetConsoleCP();

            auto restore = wil::scope_exit([&] {
                FlushConsoleInputBuffer(input);
                SetConsoleMode(input, originalMode);
                SetConsoleCP(originalCodePage);
            });

            VERIFY_WIN32_BOOL_SUCCEEDED(FlushConsoleInputBuffer(input));

            constexpr DWORD cookedMode = ENABLE_ECHO_INPUT | ENABLE_LINE_INPUT | ENABLE_PROCESSED_INPUT;
            VERIFY_WIN32_BOOL_SUCCEEDED(SetConsoleMode(input, mode == ReadMode::Raw ? 0 : cookedMode));
            VERIFY_WIN32_BOOL_SUCCEEDED(SetConsoleCP(932));

            std::wstring sendInput;
            for (const auto glyph : wide)
            {
                sendInput.append(glyph);
            }
            sendInput.append(L"\r\n");

            VERIFY_SUCCEEDED(_sendStringToInput(input, sendInput));
            contract(input, mode);
        }
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

    BEGIN_TEST_METHOD(TestCookedAliasProcessingContractReplay)
        TEST_METHOD_PROPERTY(L"TestTimeout", L"00:00:30")
        TEST_METHOD_PROPERTY(L"IsolationLevel", L"Method")
    END_TEST_METHOD()

    BEGIN_TEST_METHOD(TestCookedAlphaPermutationsContractReplay)
        TEST_METHOD_PROPERTY(L"TestTimeout", L"00:01:00")
        TEST_METHOD_PROPERTY(L"IsolationLevel", L"Method")
    END_TEST_METHOD()

    BEGIN_TEST_METHOD(TestReadCharByCharContractReplay)
        TEST_METHOD_PROPERTY(L"TestTimeout", L"00:01:00")
        TEST_METHOD_PROPERTY(L"IsolationLevel", L"Method")
    END_TEST_METHOD()

    BEGIN_TEST_METHOD(TestReadLeadTrailStringContractReplay)
        TEST_METHOD_PROPERTY(L"TestTimeout", L"00:01:00")
        TEST_METHOD_PROPERTY(L"IsolationLevel", L"Method")
    END_TEST_METHOD()

    BEGIN_TEST_METHOD(TestReadChangeCodepageInMiddleContractReplay)
        TEST_METHOD_PROPERTY(L"TestTimeout", L"00:01:00")
        TEST_METHOD_PROPERTY(L"IsolationLevel", L"Method")
    END_TEST_METHOD()

    BEGIN_TEST_METHOD(TestReadChangeCodepageBetweenBytesContractReplay)
        TEST_METHOD_PROPERTY(L"TestTimeout", L"00:01:00")
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

void InputContractReplayTests::TestCookedAliasProcessingContractReplay()
{
    const auto input = GetStdInputHandle();
    VERIFY_IS_NOT_NULL(input);

    DWORD originalMode = 0;
    VERIFY_WIN32_BOOL_SUCCEEDED(GetConsoleMode(input, &originalMode));

    auto modulePath = wil::GetModuleFileNameW<std::wstring>(nullptr);
    const auto exeName = std::filesystem::path{ modulePath }.filename().wstring();

    wchar_t aliasSource[] = L"foo";
    wchar_t aliasTarget[] = L"echo bar$Techo baz$Techo bam";
    auto mutableExeName = exeName;

    auto restoreInput = wil::scope_exit([&] {
        FlushConsoleInputBuffer(input);
        SetConsoleMode(input, originalMode);
        AddConsoleAliasW(aliasSource, nullptr, mutableExeName.data());
    });

    VERIFY_WIN32_BOOL_SUCCEEDED(FlushConsoleInputBuffer(input));

    constexpr DWORD cookedMode = ENABLE_ECHO_INPUT | ENABLE_LINE_INPUT | ENABLE_PROCESSED_INPUT;
    VERIFY_WIN32_BOOL_SUCCEEDED(SetConsoleMode(input, cookedMode));

    VERIFY_WIN32_BOOL_SUCCEEDED(AddConsoleAliasW(aliasSource, aliasTarget, mutableExeName.data()));
    VERIFY_SUCCEEDED(_sendStringToInput(input, L"foo\r\n"));

    // input2 observed CR-only command boundaries in 2020. The 2026 product emits CRLF for
    // both $T expansion and the final alias command, and ReadConsoleA exposes that CRLF.
    constexpr std::array expected{
        std::string_view{ "echo bar\r\n" },
        std::string_view{ "echo baz\r\n" },
        std::string_view{ "echo bam\r\n" },
    };

    for (const auto expectedCommand : expected)
    {
        std::string actual(500, '\0');
        VERIFY_SUCCEEDED(_readConsoleAWithTimeout(input, actual));
        VERIFY_ARE_EQUAL(std::string{ expectedCommand }, actual);
    }
}

void InputContractReplayTests::TestCookedAlphaPermutationsContractReplay()
{
    const auto input = GetStdInputHandle();
    const auto output = GetStdOutputHandle();
    VERIFY_IS_NOT_NULL(input);
    VERIFY_IS_NOT_NULL(output);

    DWORD originalInputMode = 0;
    DWORD originalOutputMode = 0;
    VERIFY_WIN32_BOOL_SUCCEEDED(GetConsoleMode(input, &originalInputMode));
    VERIFY_WIN32_BOOL_SUCCEEDED(GetConsoleMode(output, &originalOutputMode));
    const auto originalInputCodePage = GetConsoleCP();
    const auto originalOutputCodePage = GetConsoleOutputCP();

    auto restore = wil::scope_exit([&] {
        FlushConsoleInputBuffer(input);
        SetConsoleMode(input, originalInputMode);
        SetConsoleMode(output, originalOutputMode);
        SetConsoleCP(originalInputCodePage);
        SetConsoleOutputCP(originalOutputCodePage);
    });

    constexpr std::array<DWORD, 2> inputCodePages{ 437, 932 };
    constexpr std::array<DWORD, 2> outputCodePages{ 437, 932 };
    constexpr std::array<DWORD, 2> inputModes{ 0x1e7, 0x1e1 };
    constexpr DWORD outputMode = 7;

    for (const auto inputCodePage : inputCodePages)
    {
        for (const auto outputCodePage : outputCodePages)
        {
            for (const auto inputMode : inputModes)
            {
                VERIFY_WIN32_BOOL_SUCCEEDED(FlushConsoleInputBuffer(input));
                VERIFY_WIN32_BOOL_SUCCEEDED(SetConsoleMode(input, inputMode));
                VERIFY_WIN32_BOOL_SUCCEEDED(SetConsoleMode(output, outputMode));
                VERIFY_WIN32_BOOL_SUCCEEDED(SetConsoleCP(inputCodePage));
                VERIFY_WIN32_BOOL_SUCCEEDED(SetConsoleOutputCP(outputCodePage));

                std::wstring sendInput{ wide[0] };
                std::string expected{ inputCodePage == 932 ? char932[0] : char437[0] };

                if (WI_IsFlagSet(inputMode, ENABLE_LINE_INPUT))
                {
                    sendInput.append(L"\r\n");
                    expected.append(crlf);
                }

                VERIFY_SUCCEEDED(_sendStringToInput(input, sendInput));

                std::string actual(500, '\0');
                VERIFY_SUCCEEDED(_readConsoleAWithTimeout(input, actual));
                VERIFY_ARE_EQUAL(expected, actual);
            }
        }
    }

    // The historical font-dependent corruption branch was explicitly CHv1-only and marked
    // as not maintained going forward. The modern contract therefore keeps the input/output
    // code-page and line-mode matrix, without preserving obsolete font-corruption behavior.
}

void InputContractReplayTests::TestReadCharByCharContractReplay()
{
    _forEachReadMode([](const HANDLE input, const ReadMode mode) {
        for (const auto encoded : char932)
        {
            _verifyRead(input, mode, encoded.substr(0, 1));
            _verifyRead(input, mode, encoded.substr(1, 1));
        }

        _verifyRead(input, mode, crlf.substr(0, 1));
        if (mode != ReadMode::Raw)
        {
            _verifyRead(input, mode, crlf.substr(1, 1));
        }
    });
}

void InputContractReplayTests::TestReadLeadTrailStringContractReplay()
{
    _forEachReadMode([](const HANDLE input, const ReadMode mode) {
        _verifyRead(input, mode, char932[0].substr(0, 1));

        std::string expected{ char932[0].substr(1, 1) };
        expected.append(char932[1]);
        expected.append(char932[2]);
        expected.append(char932[3]);
        expected.append(crlf.substr(0, 1));

        if (mode != ReadMode::Raw)
        {
            expected.append(crlf.substr(1, 1));
        }

        _verifyRead(input, mode, expected);
    });
}

void InputContractReplayTests::TestReadChangeCodepageInMiddleContractReplay()
{
    _forEachReadMode([](const HANDLE input, const ReadMode mode) {
        std::string expectedPrefix{ char932[0] };
        expectedPrefix.append(char932[1].substr(0, 1));
        _verifyRead(input, mode, expectedPrefix);

        VERIFY_WIN32_BOOL_SUCCEEDED(SetConsoleCP(437));

        std::string expectedRemainder{ char437[2] };
        expectedRemainder.append(char437[3]);
        expectedRemainder.append(crlf.substr(0, 1));

        if (mode != ReadMode::Raw)
        {
            expectedRemainder.append(crlf.substr(1, 1));
        }

        _verifyRead(input, mode, expectedRemainder);
    });
}

void InputContractReplayTests::TestReadChangeCodepageBetweenBytesContractReplay()
{
    _forEachReadMode([](const HANDLE input, const ReadMode mode) {
        std::string expectedPrefix{ char932[0] };
        expectedPrefix.append(char932[1]);
        _verifyRead(input, mode, expectedPrefix);

        VERIFY_WIN32_BOOL_SUCCEEDED(SetConsoleCP(437));

        std::string expectedRemainder{ char437[2] };
        expectedRemainder.append(char437[3]);
        expectedRemainder.append(crlf.substr(0, 1));

        if (mode != ReadMode::Raw)
        {
            expectedRemainder.append(crlf.substr(1, 1));
        }

        _verifyRead(input, mode, expectedRemainder);
    });
}
