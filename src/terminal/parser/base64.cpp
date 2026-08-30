// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#include "precomp.h"
#include "base64.hpp"
#include "terminal_parser_ffi.h"

using namespace Microsoft::Console::VirtualTerminal;

namespace
{
    HRESULT _statusToHresult(const terminal_parser_ffi_status status) noexcept
    {
        switch (status)
        {
        case TERMINAL_PARSER_FFI_OK:
            return S_OK;
        case TERMINAL_PARSER_FFI_INVALID_BASE64:
            return HRESULT_FROM_WIN32(ERROR_INVALID_DATA);
        case TERMINAL_PARSER_FFI_INVALID_UTF8:
            return HRESULT_FROM_WIN32(ERROR_NO_UNICODE_TRANSLATION);
        case TERMINAL_PARSER_FFI_INVALID_ARGUMENT:
            return E_INVALIDARG;
        case TERMINAL_PARSER_FFI_BUFFER_TOO_SMALL:
            return HRESULT_FROM_WIN32(ERROR_INSUFFICIENT_BUFFER);
        case TERMINAL_PARSER_FFI_PANIC:
        default:
            return E_UNEXPECTED;
        }
    }
}

// Decodes an UTF8 string encoded with RFC 4648 (Base64) and returns it as UTF16 in dst.
// R09 keeps the established C++ API as a narrow compatibility facade while Rust owns the
// portable decoding semantics. The caller-owned buffer ABI prevents either language from
// freeing memory allocated by the other.
HRESULT Base64::Decode(const std::wstring_view& src, std::wstring& dst) noexcept
{
    static_assert(sizeof(wchar_t) == sizeof(uint16_t));

    size_t required{};
    const auto input = reinterpret_cast<const uint16_t*>(src.data());
    auto status = terminal_parser_ffi_base64_decode_utf16(input, src.size(), nullptr, 0, &required);

    if (status != TERMINAL_PARSER_FFI_OK && status != TERMINAL_PARSER_FFI_BUFFER_TOO_SMALL)
    {
        return _statusToHresult(status);
    }

    std::wstring decoded(required, L'\0');
    if (required != 0)
    {
        status = terminal_parser_ffi_base64_decode_utf16(
            input,
            src.size(),
            reinterpret_cast<uint16_t*>(decoded.data()),
            decoded.size(),
            &required);
        if (status != TERMINAL_PARSER_FFI_OK)
        {
            return _statusToHresult(status);
        }
        decoded.resize(required);
    }

    dst = std::move(decoded);
    return S_OK;
}
