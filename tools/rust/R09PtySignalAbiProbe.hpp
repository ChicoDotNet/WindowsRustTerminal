#pragma once

#include "terminal_parser_ffi_pty_signal.h"

#include <cstdint>
#include <cstdio>

namespace r09
{
    inline bool expect_pty_signal_read_plan(
        const uint16_t signalId,
        const uint32_t expectedKind,
        const uint32_t expectedPayloadLength)
    {
        terminal_parser_ffi_pty_signal_read_plan_result plan{};
        const auto status = terminal_parser_ffi_pty_signal_read_plan(signalId, &plan);
        if (status != TERMINAL_PARSER_FFI_OK)
        {
            std::fprintf(stderr, "PTY signal read-plan status %u for signal %u\n", static_cast<unsigned>(status), signalId);
            return false;
        }

        if (plan.kind != expectedKind || plan.payload_len != expectedPayloadLength)
        {
            std::fprintf(
                stderr,
                "PTY signal read-plan mismatch for signal %u: kind=%u payload_len=%u\n",
                signalId,
                plan.kind,
                plan.payload_len);
            return false;
        }

        return true;
    }

    inline bool pty_signal_read_plan_replay()
    {
        terminal_parser_ffi_pty_signal_read_plan_result invalid{};
        const auto unknownStatus = terminal_parser_ffi_pty_signal_read_plan(4, &invalid);
        const auto nullPlanStatus = terminal_parser_ffi_pty_signal_read_plan(1, nullptr);

        return
            expect_pty_signal_read_plan(1, TERMINAL_PARSER_FFI_PTY_SIGNAL_SHOW_HIDE_WINDOW, 2) &&
            expect_pty_signal_read_plan(2, TERMINAL_PARSER_FFI_PTY_SIGNAL_CLEAR_BUFFER, 2) &&
            expect_pty_signal_read_plan(3, TERMINAL_PARSER_FFI_PTY_SIGNAL_SET_PARENT, 8) &&
            expect_pty_signal_read_plan(8, TERMINAL_PARSER_FFI_PTY_SIGNAL_RESIZE_WINDOW, 4) &&
            unknownStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            nullPlanStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT;
    }
}
