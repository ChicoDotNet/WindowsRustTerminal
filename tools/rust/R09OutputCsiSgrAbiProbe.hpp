#pragma once

#include "terminal_parser_ffi_output_csi_sgr.h"

#include <cstddef>
#include <cstdint>
#include <cstdio>

namespace r09
{
    inline bool output_csi_sgr_replay()
    {
        const auto sgrId = static_cast<uint64_t>(static_cast<unsigned char>('m'));
        const auto unrelatedId = static_cast<uint64_t>(static_cast<unsigned char>('A'));

        size_t required = 0;
        uint32_t matched = 0;
        auto status = terminal_parser_ffi_output_csi_sgr_values(
            sgrId,
            nullptr,
            0,
            nullptr,
            0,
            &required,
            &matched);
        if (status != TERMINAL_PARSER_FFI_BUFFER_TOO_SMALL || required != 1 || matched != 1)
        {
            std::fprintf(stderr, "output CSI SGR default sizing mismatch: status=%u required=%zu matched=%u\n", static_cast<unsigned>(status), required, matched);
            return false;
        }

        int32_t defaultOutput = -1;
        status = terminal_parser_ffi_output_csi_sgr_values(
            sgrId,
            nullptr,
            0,
            &defaultOutput,
            1,
            &required,
            &matched);
        if (status != TERMINAL_PARSER_FFI_OK || required != 1 || matched != 1 || defaultOutput != 0)
        {
            std::fprintf(stderr, "output CSI SGR default payload mismatch: status=%u required=%zu matched=%u value=%d\n", static_cast<unsigned>(status), required, matched, defaultOutput);
            return false;
        }

        const int32_t singleInput[] = { 1 };
        int32_t singleOutput = -1;
        required = 0;
        matched = 0;
        status = terminal_parser_ffi_output_csi_sgr_values(
            sgrId,
            singleInput,
            1,
            &singleOutput,
            1,
            &required,
            &matched);
        if (status != TERMINAL_PARSER_FFI_OK || required != 1 || matched != 1 || singleOutput != 1)
        {
            std::fprintf(stderr, "output CSI SGR single payload mismatch: status=%u required=%zu matched=%u value=%d\n", static_cast<unsigned>(status), required, matched, singleOutput);
            return false;
        }

        const int32_t multipleInput[] = { 1, 31, 44 };
        int32_t undersizedOutput[] = { -1, -1 };
        required = 0;
        matched = 0;
        status = terminal_parser_ffi_output_csi_sgr_values(
            sgrId,
            multipleInput,
            3,
            undersizedOutput,
            2,
            &required,
            &matched);
        if (status != TERMINAL_PARSER_FFI_BUFFER_TOO_SMALL || required != 3 || matched != 1)
        {
            std::fprintf(stderr, "output CSI SGR capacity mismatch: status=%u required=%zu matched=%u\n", static_cast<unsigned>(status), required, matched);
            return false;
        }

        int32_t multipleOutput[] = { -1, -1, -1 };
        status = terminal_parser_ffi_output_csi_sgr_values(
            sgrId,
            multipleInput,
            3,
            multipleOutput,
            3,
            &required,
            &matched);
        if (status != TERMINAL_PARSER_FFI_OK || required != 3 || matched != 1 || multipleOutput[0] != 1 || multipleOutput[1] != 31 || multipleOutput[2] != 44)
        {
            std::fprintf(
                stderr,
                "output CSI SGR multi payload mismatch: status=%u required=%zu matched=%u values=%d,%d,%d\n",
                static_cast<unsigned>(status),
                required,
                matched,
                multipleOutput[0],
                multipleOutput[1],
                multipleOutput[2]);
            return false;
        }

        required = 99;
        matched = 99;
        status = terminal_parser_ffi_output_csi_sgr_values(
            unrelatedId,
            singleInput,
            1,
            nullptr,
            0,
            &required,
            &matched);
        if (status != TERMINAL_PARSER_FFI_OK || required != 0 || matched != 0)
        {
            std::fprintf(stderr, "output CSI unrelated SGR mismatch: status=%u required=%zu matched=%u\n", static_cast<unsigned>(status), required, matched);
            return false;
        }

        status = terminal_parser_ffi_output_csi_sgr_values(
            sgrId,
            nullptr,
            1,
            nullptr,
            0,
            &required,
            &matched);
        if (status != TERMINAL_PARSER_FFI_INVALID_ARGUMENT)
        {
            std::fprintf(stderr, "output CSI SGR null input mismatch: status=%u\n", static_cast<unsigned>(status));
            return false;
        }

        status = terminal_parser_ffi_output_csi_sgr_values(
            sgrId,
            singleInput,
            1,
            nullptr,
            0,
            nullptr,
            &matched);
        if (status != TERMINAL_PARSER_FFI_INVALID_ARGUMENT)
        {
            std::fprintf(stderr, "output CSI SGR null count mismatch: status=%u\n", static_cast<unsigned>(status));
            return false;
        }

        status = terminal_parser_ffi_output_csi_sgr_values(
            sgrId,
            singleInput,
            1,
            nullptr,
            0,
            &required,
            nullptr);
        if (status != TERMINAL_PARSER_FFI_INVALID_ARGUMENT)
        {
            std::fprintf(stderr, "output CSI SGR null matched mismatch: status=%u\n", static_cast<unsigned>(status));
            return false;
        }

        return true;
    }
}
