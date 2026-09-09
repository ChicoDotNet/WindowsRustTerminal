#pragma once

#include "terminal_parser_ffi_output_csi_decrqcra.h"

#include <cstdint>
#include <cstdio>

namespace r09
{
    inline uint64_t packed_csi_decrqcra_id(const char intermediate, const char finalCharacter)
    {
        return static_cast<uint64_t>(static_cast<unsigned char>(intermediate)) |
               (static_cast<uint64_t>(static_cast<unsigned char>(finalCharacter)) << 8);
    }

    inline bool expect_output_csi_decrqcra_plan(
        const char intermediate,
        const char finalCharacter,
        const int32_t requestId,
        const int32_t page,
        const int32_t bottom,
        const int32_t right,
        const uint32_t expectedKind,
        const int32_t expectedRequestId,
        const int32_t expectedPage,
        const int32_t expectedBottom,
        const int32_t expectedRight)
    {
        terminal_parser_ffi_output_csi_decrqcra_result plan{};
        const auto status = terminal_parser_ffi_output_csi_decrqcra_plan(
            packed_csi_decrqcra_id(intermediate, finalCharacter),
            requestId,
            page,
            bottom,
            right,
            &plan);
        if (status != TERMINAL_PARSER_FFI_OK)
        {
            std::fprintf(
                stderr,
                "output CSI DECRQCRA status %u for CSI %c%c\n",
                static_cast<unsigned>(status),
                intermediate,
                finalCharacter);
            return false;
        }

        if (plan.kind != expectedKind ||
            plan.request_id != expectedRequestId ||
            plan.page != expectedPage ||
            plan.bottom != expectedBottom ||
            plan.right != expectedRight)
        {
            std::fprintf(
                stderr,
                "output CSI DECRQCRA mismatch for CSI %c%c: kind=%u id=%d page=%d bottom=%d right=%d\n",
                intermediate,
                finalCharacter,
                plan.kind,
                plan.request_id,
                plan.page,
                plan.bottom,
                plan.right);
            return false;
        }

        return true;
    }

    inline bool output_csi_decrqcra_replay()
    {
        terminal_parser_ffi_output_csi_decrqcra_result invalid{};
        const auto invalidIdentifierStatus = terminal_parser_ffi_output_csi_decrqcra_plan(
            UINT64_C(0xff00000000000000), 0, 0, 0, 0, &invalid);
        const auto nullPlanStatus = terminal_parser_ffi_output_csi_decrqcra_plan(
            packed_csi_decrqcra_id('*', 'y'), 0, 0, 0, 0, nullptr);

        return
            expect_output_csi_decrqcra_plan(
                '*', 'y', 0, 0, 0, 0,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQCRA_REQUEST_CHECKSUM_RECTANGULAR_AREA,
                0, 0, 0, 0) &&
            expect_output_csi_decrqcra_plan(
                '*', 'y', 7, 2, 17, 23,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQCRA_REQUEST_CHECKSUM_RECTANGULAR_AREA,
                7, 2, 17, 23) &&
            expect_output_csi_decrqcra_plan(
                '*', 'x', 7, 2, 17, 23,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_DECRQCRA_NONE,
                0, 0, 0, 0) &&
            invalidIdentifierStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            nullPlanStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT;
    }
}
