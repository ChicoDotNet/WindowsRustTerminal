#pragma once

#include "terminal_parser_ffi_output_csi_decrqtsr.h"

#include <cstdint>
#include <cstdio>

namespace r09
{
    inline uint64_t packed_csi_decrqtsr_id(const char intermediate, const char finalCharacter)
    {
        return static_cast<uint64_t>(static_cast<unsigned char>(intermediate)) |
               (static_cast<uint64_t>(static_cast<unsigned char>(finalCharacter)) << 8);
    }

    inline bool expect_output_csi_decrqtsr_plan(
        const char intermediate,
        const char finalCharacter,
        const int32_t parameter0Raw,
        const int32_t parameter1Raw,
        const uint32_t expectedKind,
        const int32_t expectedFormat,
        const int32_t expectedFormatOption)
    {
        terminal_parser_ffi_output_csi_decrqtsr_result plan{};
        const auto status = terminal_parser_ffi_output_csi_decrqtsr_plan(
            packed_csi_decrqtsr_id(intermediate, finalCharacter),
            parameter0Raw,
            parameter1Raw,
            &plan);
        if (status != TERMINAL_PARSER_FFI_OK)
        {
            std::fprintf(
                stderr,
                "output CSI DECRQTSR status %u for CSI %c%c\n",
                static_cast<unsigned>(status),
                intermediate,
                finalCharacter);
            return false;
        }

        if (plan.kind != expectedKind ||
            plan.format != expectedFormat ||
            plan.format_option != expectedFormatOption ||
            plan.reserved0 != 0)
        {
            std::fprintf(
                stderr,
                "output CSI DECRQTSR mismatch for CSI %c%c: kind=%u format=%d option=%d reserved0=%u\n",
                intermediate,
                finalCharacter,
                plan.kind,
                plan.format,
                plan.format_option,
                plan.reserved0);
            return false;
        }

        return true;
    }

    inline bool output_csi_decrqtsr_replay()
    {
        terminal_parser_ffi_output_csi_decrqtsr_result invalid{};
        const auto invalidIdentifierStatus = terminal_parser_ffi_output_csi_decrqtsr_plan(
            UINT64_C(0xff00000000000000), -1, -1, &invalid);
        const auto nullPlanStatus = terminal_parser_ffi_output_csi_decrqtsr_plan(
            packed_csi_decrqtsr_id('$', 'u'), -1, -1, nullptr);

        return
            expect_output_csi_decrqtsr_plan(
                '$', 'u', -1, -1,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQTSR_REQUEST_TERMINAL_STATE, 0, -1) &&
            expect_output_csi_decrqtsr_plan(
                '$', 'u', 1, -1,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQTSR_REQUEST_TERMINAL_STATE, 1, -1) &&
            expect_output_csi_decrqtsr_plan(
                '$', 'u', 2, 0,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQTSR_REQUEST_TERMINAL_STATE, 2, 0) &&
            expect_output_csi_decrqtsr_plan(
                '$', 'u', 2, 7,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQTSR_REQUEST_TERMINAL_STATE, 2, 7) &&
            expect_output_csi_decrqtsr_plan(
                '$', 'w', 1, -1,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQTSR_NONE, 0, -1) &&
            invalidIdentifierStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            nullPlanStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT;
    }
}
