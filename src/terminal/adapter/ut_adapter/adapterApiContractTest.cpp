// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#include "precomp.h"
#include <wextestclass.h>
#include <stdexcept>

#include "../../../renderer/inc/DummyRenderer.hpp"
#include "adaptDispatch.hpp"

using namespace WEX::TestExecution;
using namespace Microsoft::Console::VirtualTerminal;

namespace
{
    class TerminalApiSpy final : public ITerminalApi
    {
    public:
        void UnknownSequence() noexcept override {}
        void ReturnResponse(const std::wstring_view /*response*/) override {}

        bool IsConPTY() const noexcept override { return false; }

        StateMachine& GetStateMachine() override
        {
            throw std::logic_error("GetStateMachine is outside this contract test");
        }

        BufferState GetBufferAndViewport() override
        {
            throw std::logic_error("GetBufferAndViewport is outside this contract test");
        }

        void SetViewportPosition(const til::point /*position*/) override {}
        bool IsVtInputEnabled() const override { return false; }
        void SetSystemMode(const Mode /*mode*/, const bool /*enabled*/) override {}
        bool GetSystemMode(const Mode /*mode*/) const override { return false; }

        void ReturnAnswerback() override
        {
            ++returnAnswerbackCallCount;
        }

        void WarningBell() override {}
        void SetWindowTitle(const std::wstring_view /*title*/) override {}
        void UseAlternateScreenBuffer(const TextAttribute& /*attrs*/) override {}
        void UseMainScreenBuffer() override {}
        CursorType GetUserDefaultCursorStyle() const override { return CursorType::Legacy; }
        void ShowWindow(bool /*showOrHide*/) override {}
        void SetCodePage(const unsigned int /*codepage*/) override {}
        void ResetCodePage() override {}
        unsigned int GetOutputCodePage() const override { return 0; }
        unsigned int GetInputCodePage() const override { return 0; }
        void CopyToClipboard(const wil::zwstring_view /*content*/) override {}
        void SetTaskbarProgress(const DispatchTypes::TaskbarState /*state*/, const size_t /*progress*/) override {}
        void SetWorkingDirectory(const std::wstring_view /*uri*/) override {}
        void PlayMidiNote(const int /*noteNumber*/, const int /*velocity*/, const std::chrono::microseconds /*duration*/) override {}
        bool ResizeWindow(const til::CoordType /*width*/, const til::CoordType /*height*/) override { return true; }
        void NotifyBufferRotation(const int /*delta*/) override {}
        void NotifyShellIntegrationMark() override {}
        void InvokeCompletions(std::wstring_view /*menuJson*/, unsigned int /*replaceLength*/) override {}
        void SearchMissingCommand(const std::wstring_view /*command*/) override {}
        void ShowNotification(const std::wstring_view /*title*/, const std::wstring_view /*body*/) override {}

        size_t returnAnswerbackCallCount{ 0 };
    };
}

class AdapterApiContractTest
{
public:
    TEST_CLASS(AdapterApiContractTest);

    TEST_METHOD(EnquireAnswerbackDelegatesToTerminalApi)
    {
        TerminalApiSpy api;
        DummyRenderer renderer;
        TerminalInput terminalInput;
        AdaptDispatch dispatch{ api, &renderer, renderer._renderSettings, terminalInput };

        VERIFY_ARE_EQUAL(0u, api.returnAnswerbackCallCount);

        dispatch.EnquireAnswerback();

        VERIFY_ARE_EQUAL(1u, api.returnAnswerbackCallCount);
    }
};
