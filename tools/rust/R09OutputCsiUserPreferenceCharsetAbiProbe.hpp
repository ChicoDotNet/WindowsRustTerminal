#pragma once

#include "terminal_parser_ffi_output_csi_user_preference_charset.h"

#include <cstdint>
#include <cstdio>

namespace r09
{
    inline uint64_t packed_csi_user_preference_charset_id(const char intermediate, const char finalCharacter)
    {
        return static_cast<uint64_t>(static_cast<unsigned char>(intermediate)) |
               (static_cast<uint64_t>(static_cast<unsigned char>(finalCharacter)) << 8);
    }

    inline bool expect_output_csi_user_preference_charset_plan(
        const char intermediate,
        const char finalCharacter,
        const uint32_t expectedKind)
    {
        terminal_parser_ffi_output_csi_user_preference_charset_result plan{};
        const auto status = terminal_parser_ffi_output_csi_user_preference_charset_plan(
            packed_csi_user_preference_charset_id(intermediate, finalCharacter), &plan);
        if (status != TERMINAL_PARSER_FFI_OK)
        {
            std::fprintf(
                stderr,
                "output CSI user preference charset status %u for CSI %c%c\n",
                static_cast<unsigned>(status),
                intermediate,
                finalCharacter);
            return false;
        }

        if (plan.kind != expectedKind)
        {
            std::fprintf(
                stderr,
                "output CSI user preference charset mismatch for CSI %c%c: kind=%u\n",
                intermediate,
                finalCharacter,
                plan.kind);
            return false;
        }

        return true;
    }

    inline bool output_csi_user_preference_charset_replay()
    {
        terminal_parser_ffi_output_csi_user_preference_charset_result invalidIdentifier{};
        const auto invalidIdentifierStatus = terminal_parser_ffi_output_csi_user_preference_charset_plan(
            UINT64_C(0xff00000000000000), &invalidIdentifier);
        const auto nullPlanStatus = terminal_parser_ffi_output_csi_user_preference_charset_plan(
            packed_csi_user_preference_charset_id('&', 'u'), nullptr);

        return
            expect_output_csi_user_preference_charset_plan(
                '&', 'u', TERMINAL_PARSER_FFI_OUTPUT_CSI_USER_PREFERENCE_CHARSET_REQUEST) &&
            expect_output_csi_user_preference_charset_plan(
                '$', 'u', TERMINAL_PARSER_FFI_OUTPUT_CSI_USER_PREFERENCE_CHARSET_NONE) &&
            invalidIdentifierStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT &&
            nullPlanStatus == TERMINAL_PARSER_FFI_INVALID_ARGUMENT;
    }
}
