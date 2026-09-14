#pragma once

#include "terminal_parser_ffi_output_osc.h"

#include <cstdint>
#include <cstdio>

namespace r09
{
    inline bool expect_output_osc_plan(const int32_t parameter, const uint32_t expectedKind)
    {
        terminal_parser_ffi_output_osc_plan_result plan{};
        const auto status = terminal_parser_ffi_output_osc_plan(parameter, &plan);
        if (status != TERMINAL_PARSER_FFI_OK || plan.kind != expectedKind)
        {
            std::fprintf(
                stderr,
                "output OSC mismatch: parameter=%d status=%u kind=%u expectedKind=%u\n",
                parameter,
                static_cast<unsigned>(status),
                plan.kind,
                expectedKind);
            return false;
        }
        return true;
    }

    inline bool output_osc_replay()
    {
        const struct
        {
            int32_t parameter;
            uint32_t kind;
        } cases[] = {
            { 0, TERMINAL_PARSER_FFI_OUTPUT_OSC_SET_WINDOW_TITLE },
            { 1, TERMINAL_PARSER_FFI_OUTPUT_OSC_SET_WINDOW_TITLE },
            { 2, TERMINAL_PARSER_FFI_OUTPUT_OSC_SET_WINDOW_TITLE },
            { 21, TERMINAL_PARSER_FFI_OUTPUT_OSC_SET_WINDOW_TITLE },
            { 4, TERMINAL_PARSER_FFI_OUTPUT_OSC_SET_COLOR_TABLE },
            { 10, TERMINAL_PARSER_FFI_OUTPUT_OSC_SET_DYNAMIC_COLOR },
            { 11, TERMINAL_PARSER_FFI_OUTPUT_OSC_SET_DYNAMIC_COLOR },
            { 12, TERMINAL_PARSER_FFI_OUTPUT_OSC_SET_DYNAMIC_COLOR },
            { 17, TERMINAL_PARSER_FFI_OUTPUT_OSC_SET_DYNAMIC_COLOR },
            { 52, TERMINAL_PARSER_FFI_OUTPUT_OSC_SET_CLIPBOARD },
            { 104, TERMINAL_PARSER_FFI_OUTPUT_OSC_RESET_COLOR_TABLE },
            { 110, TERMINAL_PARSER_FFI_OUTPUT_OSC_RESET_DYNAMIC_COLOR },
            { 111, TERMINAL_PARSER_FFI_OUTPUT_OSC_RESET_DYNAMIC_COLOR },
            { 112, TERMINAL_PARSER_FFI_OUTPUT_OSC_RESET_DYNAMIC_COLOR },
            { 117, TERMINAL_PARSER_FFI_OUTPUT_OSC_RESET_DYNAMIC_COLOR },
            { 7, TERMINAL_PARSER_FFI_OUTPUT_OSC_SET_CURRENT_WORKING_DIRECTORY },
            { 8, TERMINAL_PARSER_FFI_OUTPUT_OSC_HYPERLINK },
            { 9, TERMINAL_PARSER_FFI_OUTPUT_OSC_CONEMU_ACTION },
            { 133, TERMINAL_PARSER_FFI_OUTPUT_OSC_FINALTERM_ACTION },
            { 633, TERMINAL_PARSER_FFI_OUTPUT_OSC_VSCODE_ACTION },
            { 777, TERMINAL_PARSER_FFI_OUTPUT_OSC_URXVT_ACTION },
            { 1337, TERMINAL_PARSER_FFI_OUTPUT_OSC_ITERM2_ACTION },
            { 9001, TERMINAL_PARSER_FFI_OUTPUT_OSC_WT_ACTION },
            { 999, TERMINAL_PARSER_FFI_OUTPUT_OSC_NONE },
        };

        for (const auto& item : cases)
        {
            if (!expect_output_osc_plan(item.parameter, item.kind))
            {
                return false;
            }
        }

        const auto status = terminal_parser_ffi_output_osc_plan(0, nullptr);
        if (status != TERMINAL_PARSER_FFI_INVALID_ARGUMENT)
        {
            std::fprintf(stderr, "output OSC null plan mismatch: status=%u\n", static_cast<unsigned>(status));
            return false;
        }

        return true;
    }
}
