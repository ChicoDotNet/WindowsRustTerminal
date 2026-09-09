#pragma once

#include "terminal_parser_ffi_output_csi_rep.h"

#include <cstdint>
#include <cstdio>

namespace r09
{
    inline uint64_t packed_csi_rep_id(const char finalCharacter)
    {
        return static_cast<uint64_t>(static_cast<unsigned char>(finalCharacter));
    }

    inline bool expect_output_csi_rep_plan(
        const char finalCharacter,
        const int32_t parameter0,
        const uint16_t lastPrintedChar,
        const uint32_t expectedKind,
        const int32_t expectedCount)
    {
        terminal_parser_ffi_output_csi_rep_result plan{};
        const auto status = terminal_parser_ffi_output_csi_rep_plan(
            packed_csi_rep_id(finalCharacter),
            parameter0,
            lastPrintedChar,
            &plan);
        if (status != TERMINAL_PARSER_FFI_OK)
        {
            std::fprintf(
                stderr,
                "output CSI REP status %u for CSI %c\n",
                static_cast<unsigned>(status),
                finalCharacter);
            return false;
        }

        if (plan.kind != expectedKind || plan.count != expectedCount)
        {
            std::fprintf(
                stderr,
                "output CSI REP mismatch for CSI %c: kind=%u count=%d\n",
                finalCharacter,
                plan.kind,
                plan.count);
            return false;
        }

        return true;
    }

    inline bool output_csi_rep_replay()
    {
        terminal_parser_ffi_output_csi_rep_result invalid{};
        const auto invalidIdentifierStatus = terminal_parser_ffi_output_csi_rep_plan(
            UINT64_C(0xff00000000000000), 0, static_cast<uint16_t>('A'), &invalid);
        const auto nullPlanStatus = terminal_parser_ffi_output_csi_rep_plan(
            packed_csi_rep_id('b'), 0, static_cast<uint16_t>('A'), nullptr);

        return
            expect_output_csi_rep_plan(
                'b', 0, static_cast<uint16_t>('A'),
                TERMINAL_PARSER_FFI_OUTPUT_CSI_REP_REPEAT, 1) &&
            expect_output_csi_rep_plan(
                'b', 4, static_cast<uint16_t>('Z'),
                TERMINAL_PARSER_FFI_OUTPUT_CSI_REP_REPEAT, 4) &&
            expect_output_csi_rep_plan(
                'b', 7, 0,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_REP_NONE, 0) &&
            expect_output_csi_rep_plan(
                'X', 4, static_cast<uint16_t>('Z'),
                TERMINAL_PARSER_FFI_OUTPUT_CSI_REP_NONE, 0) &&
            invalidIdentifierStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            nullPlanStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT;
    }
}
