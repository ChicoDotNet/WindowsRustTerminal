#pragma once

#include <cstddef>
#include <cstdint>

extern "C"
{
    struct TerminalParserFfiConsoleArgumentToken
    {
        const std::uint16_t* data;
        std::size_t len;
    };

    struct TerminalParserFfiConsoleArgumentPlan
    {
        std::int16_t width;
        std::int16_t height;
        std::uint16_t flags;
        std::uint8_t has_server_handle;
        std::uint8_t has_signal_handle;
        std::uint32_t server_handle;
        std::uint32_t signal_handle;
        std::size_t client_commandline_len;
        std::size_t text_measurement_len;
    };

    enum TerminalParserFfiConsoleArgumentFlags : std::uint16_t
    {
        TerminalParserFfiConsoleArgumentAmbiguousIsWide = 1u << 0,
        TerminalParserFfiConsoleArgumentForceV1 = 1u << 1,
        TerminalParserFfiConsoleArgumentForceNoHandoff = 1u << 2,
        TerminalParserFfiConsoleArgumentHeadless = 1u << 3,
        TerminalParserFfiConsoleArgumentRunAsComServer = 1u << 4,
        TerminalParserFfiConsoleArgumentCreateServerHandle = 1u << 5,
        TerminalParserFfiConsoleArgumentInheritCursor = 1u << 6,
    };

    // Returns the common terminal-parser-ffi status values. Call once with
    // zero-capacity output buffers to obtain required UTF-16 lengths in
    // out_plan, then again with caller-owned buffers of those sizes.
    std::int32_t terminal_parser_ffi_console_arguments_plan(
        const TerminalParserFfiConsoleArgumentToken* tokens,
        std::size_t token_count,
        std::uint16_t* client_commandline,
        std::size_t client_commandline_capacity,
        std::uint16_t* text_measurement,
        std::size_t text_measurement_capacity,
        TerminalParserFfiConsoleArgumentPlan* out_plan);
}
