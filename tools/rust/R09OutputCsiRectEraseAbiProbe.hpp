#pragma once

#include "terminal_parser_ffi_output_csi_rect_erase.h"

#include <cstddef>
#include <cstdint>
#include <cstdio>

namespace r09
{
    inline bool output_csi_rect_erase_replay()
    {
        const auto deceraId = static_cast<uint64_t>(static_cast<unsigned char>('$')) |
                              (static_cast<uint64_t>(static_cast<unsigned char>('z')) << 8);
        const auto decseraId = static_cast<uint64_t>(static_cast<unsigned char>('$')) |
                               (static_cast<uint64_t>(static_cast<unsigned char>('{')) << 8);
        const auto unrelatedId = static_cast<uint64_t>(static_cast<unsigned char>('$')) |
                                 (static_cast<uint64_t>(static_cast<unsigned char>('x')) << 8);
        const int32_t eraseInput[] = { 1, 2, 3, 4, 99 };
        const int32_t selectiveInput[] = { 5, 6, 0, 0 };

        size_t required = 0;
        uint32_t kind = TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ERASE_NONE;
        int32_t undersizedOutput[] = { -1, -1, -1, -1 };
        auto status = terminal_parser_ffi_output_csi_rect_erase_values(
            deceraId,
            eraseInput,
            5,
            undersizedOutput,
            4,
            &required,
            &kind);
        if (status != TERMINAL_PARSER_FFI_BUFFER_TOO_SMALL || required != 5 || kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ERASE)
        {
            std::fprintf(stderr, "output CSI DECERA capacity mismatch: status=%u required=%zu kind=%u\n", static_cast<unsigned>(status), required, kind);
            return false;
        }

        int32_t eraseOutput[] = { -1, -1, -1, -1, -1 };
        status = terminal_parser_ffi_output_csi_rect_erase_values(
            deceraId,
            eraseInput,
            5,
            eraseOutput,
            5,
            &required,
            &kind);
        if (status != TERMINAL_PARSER_FFI_OK || required != 5 || kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ERASE ||
            eraseOutput[0] != 1 || eraseOutput[1] != 2 || eraseOutput[2] != 3 || eraseOutput[3] != 4 || eraseOutput[4] != 99)
        {
            std::fprintf(stderr, "output CSI DECERA payload mismatch: status=%u required=%zu kind=%u\n", static_cast<unsigned>(status), required, kind);
            return false;
        }

        int32_t selectiveOutput[] = { -1, -1, -1, -1 };
        required = 0;
        kind = TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ERASE_NONE;
        status = terminal_parser_ffi_output_csi_rect_erase_values(
            decseraId,
            selectiveInput,
            4,
            selectiveOutput,
            4,
            &required,
            &kind);
        if (status != TERMINAL_PARSER_FFI_OK || required != 4 || kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_SELECTIVE_ERASE ||
            selectiveOutput[0] != 5 || selectiveOutput[1] != 6 || selectiveOutput[2] != 0 || selectiveOutput[3] != 0)
        {
            std::fprintf(stderr, "output CSI DECSERA payload mismatch: status=%u required=%zu kind=%u\n", static_cast<unsigned>(status), required, kind);
            return false;
        }

        required = 99;
        kind = 99;
        status = terminal_parser_ffi_output_csi_rect_erase_values(
            unrelatedId,
            eraseInput,
            5,
            nullptr,
            0,
            &required,
            &kind);
        if (status != TERMINAL_PARSER_FFI_OK || required != 0 || kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_RECT_ERASE_NONE)
        {
            std::fprintf(stderr, "output CSI unrelated rectangular erase mismatch: status=%u required=%zu kind=%u\n", static_cast<unsigned>(status), required, kind);
            return false;
        }

        status = terminal_parser_ffi_output_csi_rect_erase_values(
            deceraId,
            nullptr,
            1,
            nullptr,
            0,
            &required,
            &kind);
        if (status != TERMINAL_PARSER_FFI_INVALID_ARGUMENT)
        {
            std::fprintf(stderr, "output CSI rectangular erase null input mismatch: status=%u\n", static_cast<unsigned>(status));
            return false;
        }

        status = terminal_parser_ffi_output_csi_rect_erase_values(
            0xff00'0000'0000'0000ull,
            eraseInput,
            5,
            nullptr,
            0,
            &required,
            &kind);
        if (status != TERMINAL_PARSER_FFI_INVALID_ARGUMENT)
        {
            std::fprintf(stderr, "output CSI rectangular erase identifier mismatch: status=%u\n", static_cast<unsigned>(status));
            return false;
        }

        return true;
    }
}
