#pragma once

#include "terminal_parser_ffi_output_csi_column.h"

#include <cstdint>
#include <cstdio>

namespace r09
{
    inline uint64_t packed_csi_column_id(const char finalCharacter)
    {
        return static_cast<uint64_t>(static_cast<unsigned char>('\'')) |
               (static_cast<uint64_t>(static_cast<unsigned char>(finalCharacter)) << 8);
    }

    inline bool expect_output_csi_column_plan(
        const uint64_t identifier,
        const int32_t parameter0,
        const uint32_t expectedKind,
        const int32_t expectedCount)
    {
        terminal_parser_ffi_output_csi_column_plan_result plan{};
        const auto status = terminal_parser_ffi_output_csi_column_plan(identifier, parameter0, &plan);
        if (status != TERMINAL_PARSER_FFI_OK || plan.kind != expectedKind || plan.count != expectedCount)
        {
            std::fprintf(
                stderr,
                "output CSI column mismatch: status=%u kind=%u count=%d expectedKind=%u expectedCount=%d\n",
                static_cast<unsigned>(status),
                plan.kind,
                plan.count,
                expectedKind,
                expectedCount);
            return false;
        }
        return true;
    }

    inline bool output_csi_column_replay()
    {
        const auto insertId = packed_csi_column_id('}');
        const auto deleteId = packed_csi_column_id('~');
        const auto unrelatedId = static_cast<uint64_t>(static_cast<unsigned char>('$')) |
                                 (static_cast<uint64_t>(static_cast<unsigned char>('x')) << 8);

        if (!expect_output_csi_column_plan(insertId, 0, TERMINAL_PARSER_FFI_OUTPUT_CSI_COLUMN_INSERT, 0) ||
            !expect_output_csi_column_plan(insertId, 4, TERMINAL_PARSER_FFI_OUTPUT_CSI_COLUMN_INSERT, 4) ||
            !expect_output_csi_column_plan(deleteId, 0, TERMINAL_PARSER_FFI_OUTPUT_CSI_COLUMN_DELETE, 0) ||
            !expect_output_csi_column_plan(deleteId, 7, TERMINAL_PARSER_FFI_OUTPUT_CSI_COLUMN_DELETE, 7) ||
            !expect_output_csi_column_plan(unrelatedId, 3, TERMINAL_PARSER_FFI_OUTPUT_CSI_COLUMN_NONE, 0))
        {
            return false;
        }

        auto status = terminal_parser_ffi_output_csi_column_plan(insertId, 0, nullptr);
        if (status != TERMINAL_PARSER_FFI_INVALID_ARGUMENT)
        {
            std::fprintf(stderr, "output CSI column null plan mismatch: status=%u\n", static_cast<unsigned>(status));
            return false;
        }

        terminal_parser_ffi_output_csi_column_plan_result plan{};
        status = terminal_parser_ffi_output_csi_column_plan(0xff00'0000'0000'0000ULL, 0, &plan);
        if (status != TERMINAL_PARSER_FFI_INVALID_ARGUMENT)
        {
            std::fprintf(stderr, "output CSI column invalid identifier mismatch: status=%u\n", static_cast<unsigned>(status));
            return false;
        }

        return true;
    }
}
