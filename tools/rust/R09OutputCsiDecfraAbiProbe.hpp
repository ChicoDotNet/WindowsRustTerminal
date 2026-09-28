#pragma once

#include "terminal_parser_ffi_output_csi_decfra.h"

#include <cstddef>
#include <cstdint>
#include <cstdio>

namespace r09
{
    inline bool output_csi_decfra_replay()
    {
        const auto decfraId = static_cast<uint64_t>(static_cast<unsigned char>('$')) |
                              (static_cast<uint64_t>(static_cast<unsigned char>('x')) << 8);
        const auto unrelatedId = static_cast<uint64_t>(static_cast<unsigned char>('$')) |
                                 (static_cast<uint64_t>(static_cast<unsigned char>('r')) << 8);
        const int32_t input[] = { 65, 1, 2, 3, 4 };

        size_t required = 0;
        uint32_t matched = 0;
        int32_t undersizedOutput[] = { -1, -1, -1, -1 };
        auto status = terminal_parser_ffi_output_csi_decfra_values(
            decfraId,
            input,
            5,
            undersizedOutput,
            4,
            &required,
            &matched);
        if (status != TERMINAL_PARSER_FFI_BUFFER_TOO_SMALL || required != 5 || matched != 1)
        {
            std::fprintf(stderr, "output CSI DECFRA capacity mismatch: status=%u required=%zu matched=%u\n", static_cast<unsigned>(status), required, matched);
            return false;
        }

        int32_t output[] = { -1, -1, -1, -1, -1 };
        status = terminal_parser_ffi_output_csi_decfra_values(
            decfraId,
            input,
            5,
            output,
            5,
            &required,
            &matched);
        if (status != TERMINAL_PARSER_FFI_OK || required != 5 || matched != 1 ||
            output[0] != 65 || output[1] != 1 || output[2] != 2 || output[3] != 3 || output[4] != 4)
        {
            std::fprintf(
                stderr,
                "output CSI DECFRA payload mismatch: status=%u required=%zu matched=%u values=%d,%d,%d,%d,%d\n",
                static_cast<unsigned>(status),
                required,
                matched,
                output[0],
                output[1],
                output[2],
                output[3],
                output[4]);
            return false;
        }

        required = 99;
        matched = 99;
        status = terminal_parser_ffi_output_csi_decfra_values(
            unrelatedId,
            input,
            5,
            nullptr,
            0,
            &required,
            &matched);
        if (status != TERMINAL_PARSER_FFI_OK || required != 0 || matched != 0)
        {
            std::fprintf(stderr, "output CSI unrelated DECFRA mismatch: status=%u required=%zu matched=%u\n", static_cast<unsigned>(status), required, matched);
            return false;
        }

        status = terminal_parser_ffi_output_csi_decfra_values(
            decfraId,
            nullptr,
            1,
            nullptr,
            0,
            &required,
            &matched);
        if (status != TERMINAL_PARSER_FFI_INVALID_ARGUMENT)
        {
            std::fprintf(stderr, "output CSI DECFRA null input mismatch: status=%u\n", static_cast<unsigned>(status));
            return false;
        }

        status = terminal_parser_ffi_output_csi_decfra_values(
            decfraId,
            input,
            5,
            nullptr,
            0,
            nullptr,
            &matched);
        if (status != TERMINAL_PARSER_FFI_INVALID_ARGUMENT)
        {
            std::fprintf(stderr, "output CSI DECFRA null count mismatch: status=%u\n", static_cast<unsigned>(status));
            return false;
        }

        status = terminal_parser_ffi_output_csi_decfra_values(
            decfraId,
            input,
            5,
            nullptr,
            0,
            &required,
            nullptr);
        if (status != TERMINAL_PARSER_FFI_INVALID_ARGUMENT)
        {
            std::fprintf(stderr, "output CSI DECFRA null matched mismatch: status=%u\n", static_cast<unsigned>(status));
            return false;
        }

        return true;
    }
}
