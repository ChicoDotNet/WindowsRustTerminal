#pragma once

#include "terminal_parser_ffi_output_csi_pop_sgr.h"

#include <cstdint>
#include <cstdio>

namespace r09
{
    inline uint64_t packed_csi_pop_sgr_id(const char suffix)
    {
        return static_cast<uint64_t>(static_cast<unsigned char>('#')) |
               (static_cast<uint64_t>(static_cast<unsigned char>(suffix)) << 8);
    }

    inline bool output_csi_pop_sgr_replay()
    {
        for (const char suffix : { '}', 'q' })
        {
            terminal_parser_ffi_output_csi_pop_sgr_result plan{};
            const auto status = terminal_parser_ffi_output_csi_pop_sgr_plan(
                packed_csi_pop_sgr_id(suffix), &plan);
            if (status != TERMINAL_PARSER_FFI_OK ||
                plan.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_POP_SGR_POP)
            {
                std::fprintf(
                    stderr,
                    "output CSI pop SGR mismatch for #%c: status=%u kind=%u\n",
                    suffix,
                    static_cast<unsigned>(status),
                    plan.kind);
                return false;
            }
        }

        terminal_parser_ffi_output_csi_pop_sgr_result unrelated{};
        const auto unrelatedStatus = terminal_parser_ffi_output_csi_pop_sgr_plan(
            static_cast<uint64_t>(static_cast<unsigned char>('m')), &unrelated);
        if (unrelatedStatus != TERMINAL_PARSER_FFI_OK ||
            unrelated.kind != TERMINAL_PARSER_FFI_OUTPUT_CSI_POP_SGR_NONE)
        {
            std::fprintf(
                stderr,
                "output CSI unrelated pop SGR mismatch: status=%u kind=%u\n",
                static_cast<unsigned>(unrelatedStatus),
                unrelated.kind);
            return false;
        }

        const auto nullStatus = terminal_parser_ffi_output_csi_pop_sgr_plan(
            packed_csi_pop_sgr_id('}'), nullptr);
        if (nullStatus != TERMINAL_PARSER_FFI_INVALID_ARGUMENT)
        {
            std::fprintf(
                stderr,
                "output CSI pop SGR null pointer mismatch: status=%u\n",
                static_cast<unsigned>(nullStatus));
            return false;
        }

        return true;
    }
}
