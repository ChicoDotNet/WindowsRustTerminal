#pragma once

#include "terminal_parser_ffi_output_csi_decac.h"

#include <cstdint>
#include <cstdio>

namespace r09
{
    inline uint64_t packed_csi_decac_id(const char intermediate, const char finalCharacter)
    {
        return static_cast<uint64_t>(static_cast<unsigned char>(intermediate)) |
               (static_cast<uint64_t>(static_cast<unsigned char>(finalCharacter)) << 8);
    }

    inline bool expect_output_csi_decac_plan(
        const char intermediate,
        const char finalCharacter,
        const int32_t parameter0,
        const int32_t parameter1,
        const int32_t parameter2,
        const uint32_t expectedKind,
        const int32_t expectedItem,
        const int32_t expectedForegroundIndex,
        const int32_t expectedBackgroundIndex)
    {
        terminal_parser_ffi_output_csi_decac_result plan{};
        const auto status = terminal_parser_ffi_output_csi_decac_plan(
            packed_csi_decac_id(intermediate, finalCharacter),
            parameter0,
            parameter1,
            parameter2,
            &plan);
        if (status != TERMINAL_PARSER_FFI_OK)
        {
            std::fprintf(
                stderr,
                "output CSI DECAC status %u for CSI %c%c\n",
                static_cast<unsigned>(status),
                intermediate,
                finalCharacter);
            return false;
        }

        if (plan.kind != expectedKind ||
            plan.item != expectedItem ||
            plan.foreground_index != expectedForegroundIndex ||
            plan.background_index != expectedBackgroundIndex)
        {
            std::fprintf(
                stderr,
                "output CSI DECAC mismatch for CSI %c%c: kind=%u item=%d foreground=%d background=%d\n",
                intermediate,
                finalCharacter,
                plan.kind,
                plan.item,
                plan.foreground_index,
                plan.background_index);
            return false;
        }

        return true;
    }

    inline bool output_csi_decac_replay()
    {
        terminal_parser_ffi_output_csi_decac_result invalid{};
        const auto invalidIdentifierStatus = terminal_parser_ffi_output_csi_decac_plan(
            UINT64_C(0xff00000000000000), 0, 0, 0, &invalid);
        const auto nullPlanStatus = terminal_parser_ffi_output_csi_decac_plan(
            packed_csi_decac_id(',', '|'), 0, 0, 0, nullptr);

        return
            expect_output_csi_decac_plan(
                ',', '|', 0, 0, 0,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_DECAC_ASSIGN_COLOR, 0, 0, 0) &&
            expect_output_csi_decac_plan(
                ',', '|', 1, 7, 4,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_DECAC_ASSIGN_COLOR, 1, 7, 4) &&
            expect_output_csi_decac_plan(
                ',', '|', 2, 15, 0,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_DECAC_ASSIGN_COLOR, 2, 15, 0) &&
            expect_output_csi_decac_plan(
                ',', '~', 1, 7, 4,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_DECAC_NONE, 0, 0, 0) &&
            invalidIdentifierStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            nullPlanStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT;
    }
}
