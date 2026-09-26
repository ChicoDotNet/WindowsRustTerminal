// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT license.

#pragma once

#include "OutputStateMachineEngine.hpp"
#include "RustStateMachineBridge.hpp"

#include <memory>
#include <string_view>
#include <utility>

namespace Microsoft::Console::VirtualTerminal
{
    // Product-facing owner for terminal output parsing.
    //
    // Rust owns the portable parser semantics. The native output engine remains
    // the narrow Windows/native dispatch seam and is deliberately owned beside
    // the Rust bridge so callers never need to recover it from a C++ parser.
    class TerminalOutputStateMachine final
    {
    public:
        explicit TerminalOutputStateMachine(std::unique_ptr<ITermDispatch> dispatch) :
            _engine{ std::move(dispatch) },
            _parser{ _engine }
        {
        }

        TerminalOutputStateMachine(const TerminalOutputStateMachine&) = delete;
        TerminalOutputStateMachine& operator=(const TerminalOutputStateMachine&) = delete;
        TerminalOutputStateMachine(TerminalOutputStateMachine&&) = delete;
        TerminalOutputStateMachine& operator=(TerminalOutputStateMachine&&) = delete;

        [[nodiscard]] bool Initialize() noexcept
        {
            return _parser.Initialize() == TERMINAL_PARSER_FFI_OK;
        }

        [[nodiscard]] bool Process(const std::wstring_view text) noexcept
        {
            return _parser.Process(text) == TERMINAL_PARSER_FFI_OK;
        }

        [[nodiscard]] bool Reset() noexcept
        {
            return _parser.Reset() == TERMINAL_PARSER_FFI_OK;
        }

        [[nodiscard]] bool SetParserMode(const terminal_parser_ffi_parser_mode mode, const bool enabled) noexcept
        {
            return _parser.SetParserMode(mode, enabled) == TERMINAL_PARSER_FFI_OK;
        }

        [[nodiscard]] bool GetParserMode(const terminal_parser_ffi_parser_mode mode, bool& enabled) const noexcept
        {
            return _parser.GetParserMode(mode, enabled) == TERMINAL_PARSER_FFI_OK;
        }

        ITermDispatch& Dispatch() noexcept
        {
            return _engine.Dispatch();
        }

        const ITermDispatch& Dispatch() const noexcept
        {
            return _engine.Dispatch();
        }

    private:
        OutputStateMachineEngine _engine;
        RustStateMachineBridge _parser;
    };
}
