#pragma once

#include "terminal_parser_ffi_output_csi_decsca.h"

#include <cstddef>
#include <cstdint>
#include <cstdio>

namespace r09
{
    inline uint64_t packed_csi_decsca_id()
    {
        return static_cast<uint64_t>(static_cast<unsigned char>('"')) |
               (static_cast<uint64_t>(static_cast<unsigned char>('q')) << 8);
    }

    inline bool output_csi_decsca_replay()
    {
        const auto id = packed_csi_decsca_id();

        const int32_t singleInput[] = { 0 };
        size_t required = 0;
        auto status = terminal_parser_ffi_output_csi_decsca_values(
            id,
            singleInput,
            1,
            nullptr,
            0,
            &required);
        if (status != TERMINAL_PARSER_FFI_BUFFER_TOO_SMALL || required != 1)
        {
            std::fprintf(stderr, "output CSI DECSCA sizing mismatch: status=%u required=%zu\n", static_cast<unsigned>(status), required);
            return false;
        }

        int32_t singleOutput = -1;
        status = terminal_parser_ffi_output_csi_decsca_values(
            id,
            singleInput,
            1,
            &singleOutput,
            1,
            &required);
        if (status != TERMINAL_PARSER_FFI_OK || required != 1 || singleOutput != 0)
        {
            std::fprintf(stderr, "output CSI DECSCA single payload mismatch: status=%u required=%zu value=%d\n", static_cast<unsigned>(status), required, singleOutput);
            return false;
        }

        const int32_t multipleInput[] = { 1, 2 };
        int32_t undersizedOutput = -1;
        required = 0;
        status = terminal_parser_ffi_output_csi_decsca_values(
            id,
            multipleInput,
            2,
            &undersizedOutput,
            1,
            &required);
        if (status != TERMINAL_PARSER_FFI_BUFFER_TOO_SMALL || required != 2)
        {
            std::fprintf(stderr, "output CSI DECSCA capacity mismatch: status=%u required=%zu\n", static_cast<unsigned>(status), required);
            return false;
        }

        int32_t multipleOutput[] = { -1, -1 };
        status = terminal_parser_ffi_output_csi_decsca_values(
            id,
            multipleInput,
            2,
            multipleOutput,
            2,
            &required);
        if (status != TERMINAL_PARSER_FFI_OK || required != 2 || multipleOutput[0] != 1 || multipleOutput[1] != 2)
        {
            std::fprintf(
                stderr,
                "output CSI DECSCA multiple payload mismatch: status=%u required=%zu values=%d,%d\n",
                static_cast<unsigned>(status),
                required,
                multipleOutput[0],
                multipleOutput[1]);
            return false;
        }

        required = 99;
        status = terminal_parser_ffi_output_csi_decsca_values(
            static_cast<uint64_t>(static_cast<unsigned char>('m')),
            singleInput,
            1,
            nullptr,
            0,
            &required);
        if (status != TERMINAL_PARSER_FFI_OK || required != 0)
        {
            std::fprintf(stderr, "output CSI unrelated DECSCA mismatch: status=%u required=%zu\n", static_cast<unsigned>(status), required);
            return false;
        }

        status = terminal_parser_ffi_output_csi_decsca_values(
            id,
            nullptr,
            1,
            nullptr,
            0,
            &required);
        if (status != TERMINAL_PARSER_FFI_INVALID_ARGUMENT)
        {
            std::fprintf(stderr, "output CSI DECSCA null input mismatch: status=%u\n", static_cast<unsigned>(status));
            return false;
        }

        status = terminal_parser_ffi_output_csi_decsca_values(
            id,
            singleInput,
            1,
            nullptr,
            0,
            nullptr);
        if (status != TERMINAL_PARSER_FFI_INVALID_ARGUMENT)
        {
            std::fprintf(stderr, "output CSI DECSCA null count mismatch: status=%u\n", static_cast<unsigned>(status));
            return false;
        }

        return true;
    }
}
