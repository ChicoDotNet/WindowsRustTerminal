#pragma once

#include "terminal_parser_ffi_output_dcs.h"

#include <cstdint>
#include <cstdio>
#include <string_view>

namespace r09
{
    inline uint64_t packed_dcs_id(const std::string_view text)
    {
        uint64_t value = 0;
        for (size_t index = 0; index < text.size(); ++index)
        {
            value |= static_cast<uint64_t>(static_cast<unsigned char>(text[index])) << (index * 8);
        }
        return value;
    }

    inline bool expect_output_dcs_plan(const std::string_view id, const uint32_t expectedKind)
    {
        terminal_parser_ffi_output_dcs_plan_result plan{};
        const auto status = terminal_parser_ffi_output_dcs_plan(packed_dcs_id(id), &plan);
        if (status != TERMINAL_PARSER_FFI_OK || plan.kind != expectedKind)
        {
            std::fprintf(
                stderr,
                "output DCS mismatch: status=%u kind=%u expectedKind=%u\n",
                static_cast<unsigned>(status),
                plan.kind,
                expectedKind);
            return false;
        }
        return true;
    }

    inline bool output_dcs_replay()
    {
        if (!expect_output_dcs_plan("q", TERMINAL_PARSER_FFI_OUTPUT_DCS_DEFINE_SIXEL_IMAGE) ||
            !expect_output_dcs_plan("{", TERMINAL_PARSER_FFI_OUTPUT_DCS_DOWNLOAD_DRCS) ||
            !expect_output_dcs_plan("!u", TERMINAL_PARSER_FFI_OUTPUT_DCS_ASSIGN_USER_PREFERENCE_CHARSET) ||
            !expect_output_dcs_plan("!z", TERMINAL_PARSER_FFI_OUTPUT_DCS_DEFINE_MACRO) ||
            !expect_output_dcs_plan("$p", TERMINAL_PARSER_FFI_OUTPUT_DCS_RESTORE_TERMINAL_STATE) ||
            !expect_output_dcs_plan("$q", TERMINAL_PARSER_FFI_OUTPUT_DCS_REQUEST_SETTING) ||
            !expect_output_dcs_plan("$t", TERMINAL_PARSER_FFI_OUTPUT_DCS_RESTORE_PRESENTATION_STATE) ||
            !expect_output_dcs_plan("x", TERMINAL_PARSER_FFI_OUTPUT_DCS_NONE))
        {
            return false;
        }

        auto status = terminal_parser_ffi_output_dcs_plan(packed_dcs_id("q"), nullptr);
        if (status != TERMINAL_PARSER_FFI_INVALID_ARGUMENT)
        {
            std::fprintf(stderr, "output DCS null plan mismatch: status=%u\n", static_cast<unsigned>(status));
            return false;
        }

        terminal_parser_ffi_output_dcs_plan_result plan{};
        status = terminal_parser_ffi_output_dcs_plan(0xff00'0000'0000'0000ULL, &plan);
        if (status != TERMINAL_PARSER_FFI_INVALID_ARGUMENT)
        {
            std::fprintf(stderr, "output DCS invalid identifier mismatch: status=%u\n", static_cast<unsigned>(status));
            return false;
        }

        return true;
    }
}