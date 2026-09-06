#pragma once

#include "terminal_parser_ffi_output_csi_rect_copy.h"

#include <cstddef>
#include <cstdint>
#include <cstdio>

namespace r09
{
    inline bool output_csi_rect_copy_replay()
    {
        const auto deccraId = static_cast<uint64_t>(static_cast<unsigned char>('$')) |
                              (static_cast<uint64_t>(static_cast<unsigned char>('v')) << 8);
        const auto unrelatedId = static_cast<uint64_t>(static_cast<unsigned char>('$')) |
                                 (static_cast<uint64_t>(static_cast<unsigned char>('x')) << 8);
        const int32_t input[] = { 1, 2, 3, 4, 5, 6, 7, 8, 99 };

        size_t required = 0;
        uint32_t matched = 0;
        int32_t undersizedOutput[] = { -1, -1, -1, -1, -1, -1, -1, -1 };
        auto status = terminal_parser_ffi_output_csi_rect_copy_values(
            deccraId,
            input,
            9,
            undersizedOutput,
            8,
            &required,
            &matched);
        if (status != TERMINAL_PARSER_FFI_BUFFER_TOO_SMALL || required != 9 || matched != 1)
        {
            std::fprintf(stderr, "output CSI DECCRA capacity mismatch: status=%u required=%zu matched=%u\n", static_cast<unsigned>(status), required, matched);
            return false;
        }

        int32_t output[] = { -1, -1, -1, -1, -1, -1, -1, -1, -1 };
        status = terminal_parser_ffi_output_csi_rect_copy_values(
            deccraId,
            input,
            9,
            output,
            9,
            &required,
            &matched);
        if (status != TERMINAL_PARSER_FFI_OK || required != 9 || matched != 1)
        {
            std::fprintf(stderr, "output CSI DECCRA metadata mismatch: status=%u required=%zu matched=%u\n", static_cast<unsigned>(status), required, matched);
            return false;
        }
        for (size_t index = 0; index < 9; ++index)
        {
            if (output[index] != input[index])
            {
                std::fprintf(stderr, "output CSI DECCRA payload mismatch at %zu: actual=%d expected=%d\n", index, output[index], input[index]);
                return false;
            }
        }

        required = 99;
        matched = 99;
        status = terminal_parser_ffi_output_csi_rect_copy_values(
            unrelatedId,
            input,
            9,
            nullptr,
            0,
            &required,
            &matched);
        if (status != TERMINAL_PARSER_FFI_OK || required != 0 || matched != 0)
        {
            std::fprintf(stderr, "output CSI unrelated rectangular copy mismatch: status=%u required=%zu matched=%u\n", static_cast<unsigned>(status), required, matched);
            return false;
        }

        status = terminal_parser_ffi_output_csi_rect_copy_values(
            deccraId,
            nullptr,
            1,
            nullptr,
            0,
            &required,
            &matched);
        if (status != TERMINAL_PARSER_FFI_INVALID_ARGUMENT)
        {
            std::fprintf(stderr, "output CSI rectangular copy null input mismatch: status=%u\n", static_cast<unsigned>(status));
            return false;
        }

        status = terminal_parser_ffi_output_csi_rect_copy_values(
            0xff00'0000'0000'0000ull,
            input,
            9,
            nullptr,
            0,
            &required,
            &matched);
        if (status != TERMINAL_PARSER_FFI_INVALID_ARGUMENT)
        {
            std::fprintf(stderr, "output CSI rectangular copy identifier mismatch: status=%u\n", static_cast<unsigned>(status));
            return false;
        }

        return true;
    }
}
