#pragma once

#include "terminal_parser_ffi_output_csi_push_sgr.h"

#include <cstddef>
#include <cstdint>
#include <cstdio>

namespace r09
{
    inline uint64_t packed_csi_push_sgr_id(const char final)
    {
        return static_cast<uint64_t>(static_cast<unsigned char>('#')) |
               (static_cast<uint64_t>(static_cast<unsigned char>(final)) << 8);
    }

    inline bool output_csi_push_sgr_replay()
    {
        const auto primaryId = packed_csi_push_sgr_id('{');
        const auto aliasId = packed_csi_push_sgr_id('p');

        const int32_t singleInput[] = { 1 };
        size_t required = 0;
        auto status = terminal_parser_ffi_output_csi_push_sgr_values(
            primaryId,
            singleInput,
            1,
            nullptr,
            0,
            &required);
        if (status != TERMINAL_PARSER_FFI_BUFFER_TOO_SMALL || required != 1)
        {
            std::fprintf(stderr, "output CSI push SGR sizing mismatch: status=%u required=%zu\n", static_cast<unsigned>(status), required);
            return false;
        }

        int32_t singleOutput = -1;
        status = terminal_parser_ffi_output_csi_push_sgr_values(
            primaryId,
            singleInput,
            1,
            &singleOutput,
            1,
            &required);
        if (status != TERMINAL_PARSER_FFI_OK || required != 1 || singleOutput != 1)
        {
            std::fprintf(stderr, "output CSI push SGR primary payload mismatch: status=%u required=%zu value=%d\n", static_cast<unsigned>(status), required, singleOutput);
            return false;
        }

        const int32_t multipleInput[] = { 1, 2 };
        int32_t undersizedOutput = -1;
        required = 0;
        status = terminal_parser_ffi_output_csi_push_sgr_values(
            aliasId,
            multipleInput,
            2,
            &undersizedOutput,
            1,
            &required);
        if (status != TERMINAL_PARSER_FFI_BUFFER_TOO_SMALL || required != 2)
        {
            std::fprintf(stderr, "output CSI push SGR alias capacity mismatch: status=%u required=%zu\n", static_cast<unsigned>(status), required);
            return false;
        }

        int32_t multipleOutput[] = { -1, -1 };
        status = terminal_parser_ffi_output_csi_push_sgr_values(
            aliasId,
            multipleInput,
            2,
            multipleOutput,
            2,
            &required);
        if (status != TERMINAL_PARSER_FFI_OK || required != 2 || multipleOutput[0] != 1 || multipleOutput[1] != 2)
        {
            std::fprintf(
                stderr,
                "output CSI push SGR alias payload mismatch: status=%u required=%zu values=%d,%d\n",
                static_cast<unsigned>(status),
                required,
                multipleOutput[0],
                multipleOutput[1]);
            return false;
        }

        required = 99;
        status = terminal_parser_ffi_output_csi_push_sgr_values(
            static_cast<uint64_t>(static_cast<unsigned char>('m')),
            singleInput,
            1,
            nullptr,
            0,
            &required);
        if (status != TERMINAL_PARSER_FFI_OK || required != 0)
        {
            std::fprintf(stderr, "output CSI unrelated push SGR mismatch: status=%u required=%zu\n", static_cast<unsigned>(status), required);
            return false;
        }

        status = terminal_parser_ffi_output_csi_push_sgr_values(
            primaryId,
            nullptr,
            1,
            nullptr,
            0,
            &required);
        if (status != TERMINAL_PARSER_FFI_INVALID_ARGUMENT)
        {
            std::fprintf(stderr, "output CSI push SGR null input mismatch: status=%u\n", static_cast<unsigned>(status));
            return false;
        }

        status = terminal_parser_ffi_output_csi_push_sgr_values(
            primaryId,
            singleInput,
            1,
            nullptr,
            0,
            nullptr);
        if (status != TERMINAL_PARSER_FFI_INVALID_ARGUMENT)
        {
            std::fprintf(stderr, "output CSI push SGR null count mismatch: status=%u\n", static_cast<unsigned>(status));
            return false;
        }

        return true;
    }
}
