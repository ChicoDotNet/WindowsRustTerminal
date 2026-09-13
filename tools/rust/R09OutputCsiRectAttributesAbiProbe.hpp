#pragma once

#include "terminal_parser_ffi_output_csi_rect_attributes.h"

#include <cstdint>
#include <cstdio>

namespace r09
{
    inline uint64_t packed_csi_rect_attributes_id(const char intermediate, const char finalCharacter)
    {
        return static_cast<uint64_t>(static_cast<unsigned char>(intermediate)) |
               (static_cast<uint64_t>(static_cast<unsigned char>(finalCharacter)) << 8);
    }

    inline bool expect_output_csi_rect_attributes_plan(
        const char intermediate,
        const char finalCharacter,
        const int32_t bottom,
        const int32_t right,
        const uint32_t expectedKind,
        const int32_t expectedBottom,
        const int32_t expectedRight)
    {
        terminal_parser_ffi_output_csi_rect_attributes_result plan{};
        const auto status = terminal_parser_ffi_output_csi_rect_attributes_plan(
            packed_csi_rect_attributes_id(intermediate, finalCharacter),
            bottom,
            right,
            &plan);
        if (status != TERMINAL_PARSER_FFI_OK)
        {
            std::fprintf(
                stderr,
                "output CSI rectangular attributes status %u for CSI %c%c\n",
                static_cast<unsigned>(status),
                intermediate,
                finalCharacter);
            return false;
        }

        if (plan.kind != expectedKind || plan.bottom != expectedBottom || plan.right != expectedRight)
        {
            std::fprintf(
                stderr,
                "output CSI rectangular attributes mismatch for CSI %c%c: kind=%u bottom=%d right=%d\n",
                intermediate,
                finalCharacter,
                plan.kind,
                plan.bottom,
                plan.right);
            return false;
        }

        return true;
    }

    inline bool output_csi_rect_attributes_replay()
    {
        terminal_parser_ffi_output_csi_rect_attributes_result invalid{};
        const auto invalidIdentifierStatus = terminal_parser_ffi_output_csi_rect_attributes_plan(
            UINT64_C(0xff00000000000000), 0, 0, &invalid);
        const auto nullPlanStatus = terminal_parser_ffi_output_csi_rect_attributes_plan(
            packed_csi_rect_attributes_id('$', 'r'), 0, 0, nullptr);

        return
            expect_output_csi_rect_attributes_plan(
                '$', 'r', 0, 0,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_CHANGE, 0, 0) &&
            expect_output_csi_rect_attributes_plan(
                '$', 'r', 17, 23,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_CHANGE, 17, 23) &&
            expect_output_csi_rect_attributes_plan(
                '$', 't', 0, 0,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_REVERSE, 0, 0) &&
            expect_output_csi_rect_attributes_plan(
                '$', 't', 17, 23,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_REVERSE, 17, 23) &&
            expect_output_csi_rect_attributes_plan(
                '$', 'u', 17, 23,
                TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ATTRIBUTES_NONE, 0, 0) &&
            invalidIdentifierStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            nullPlanStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT;
    }
}
