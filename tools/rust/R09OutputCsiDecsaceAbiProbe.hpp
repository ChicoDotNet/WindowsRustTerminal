#pragma once

#include "terminal_parser_ffi_output_csi_decsace.h"

#include <cstdint>
#include <cstdio>

namespace r09
{
    inline uint64_t packed_csi_decsace_id(const char intermediate, const char finalCharacter)
    {
        return static_cast<uint64_t>(static_cast<unsigned char>(intermediate)) |
               (static_cast<uint64_t>(static_cast<unsigned char>(finalCharacter)) << 8);
    }

    inline bool expect_output_csi_decsace_plan(
        const char intermediate,
        const char finalCharacter,
        const int32_t parameter0,
        const uint32_t expectedKind,
        const int32_t expectedChangeExtent)
    {
        terminal_parser_ffi_output_csi_decsace_result plan{};
        const auto status = terminal_parser_ffi_output_csi_decsace_plan(
            packed_csi_decsace_id(intermediate, finalCharacter),
            parameter0,
            &plan);
        if (status != TERMINAL_PARSER_FFI_OK)
        {
            std::fprintf(
                stderr,
                "output CSI DECSACE status %u for CSI %c%c\n",
                static_cast<unsigned>(status),
                intermediate,
                finalCharacter);
            return false;
        }

        if (plan.kind != expectedKind ||
            plan.change_extent != expectedChangeExtent ||
            plan.reserved0 != 0 ||
            plan.reserved1 != 0)
        {
            std::fprintf(
                stderr,
                "output CSI DECSACE mismatch for CSI %c%c: kind=%u change_extent=%d reserved0=%u reserved1=%u\n",
                intermediate,
                finalCharacter,
                plan.kind,
                plan.change_extent,
                plan.reserved0,
                plan.reserved1);
            return false;
        }

        return true;
    }

    inline bool output_csi_decsace_replay()
    {
        terminal_parser_ffi_output_csi_decsace_result invalid{};
        const auto invalidIdentifierStatus = terminal_parser_ffi_output_csi_decsace_plan(
            UINT64_C(0xff00000000000000), 0, &invalid);
        const auto nullPlanStatus = terminal_parser_ffi_output_csi_decsace_plan(
            packed_csi_decsace_id('*', 'x'), 0, nullptr);

        return
            expect_output_csi_decsace_plan(
                '*', 'x', 0,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_DECSACE_SELECT_ATTRIBUTE_CHANGE_EXTENT, 0) &&
            expect_output_csi_decsace_plan(
                '*', 'x', 1,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_DECSACE_SELECT_ATTRIBUTE_CHANGE_EXTENT, 1) &&
            expect_output_csi_decsace_plan(
                '*', 'x', 2,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_DECSACE_SELECT_ATTRIBUTE_CHANGE_EXTENT, 2) &&
            expect_output_csi_decsace_plan(
                '*', 'y', 1,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_DECSACE_NONE, 0) &&
            invalidIdentifierStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            nullPlanStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT;
    }
}
