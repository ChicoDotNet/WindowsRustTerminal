// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#pragma once

#include "IStateMachineEngine.hpp"
#include "../../../rust/terminal-parser-ffi/include/terminal_parser_ffi.h"

#include <limits>
#include <span>
#include <utility>
#include <vector>

namespace Microsoft::Console::VirtualTerminal
{
    // Narrow adapter from the Rust parser ABI back into the existing native engine.
    // It intentionally owns no parsing semantics. The vectors only materialize the
    // borrowed FFI parameter view for the duration of an engine dispatch.
    class RustStateMachineBridge final
    {
    public:
        explicit RustStateMachineBridge(IStateMachineEngine& engine) noexcept :
            _engine{ engine }
        {
        }

        static bool CsiLosslessCallback(void* userData,
                                        const uint64_t id,
                                        const int32_t* values,
                                        const uint8_t* present,
                                        const size_t parameterCount,
                                        const int32_t* subValues,
                                        const uint8_t* subPresent,
                                        const size_t subParameterCount,
                                        const size_t* subOffsets,
                                        const size_t* subCounts)
        {
            if (userData == nullptr)
            {
                return false;
            }

            return static_cast<RustStateMachineBridge*>(userData)->_DispatchCsi(
                id,
                values,
                present,
                parameterCount,
                subValues,
                subPresent,
                subParameterCount,
                subOffsets,
                subCounts);
        }

    private:
        bool _DispatchCsi(const uint64_t id,
                          const int32_t* values,
                          const uint8_t* present,
                          const size_t parameterCount,
                          const int32_t* subValues,
                          const uint8_t* subPresent,
                          const size_t subParameterCount,
                          const size_t* subOffsets,
                          const size_t* subCounts)
        {
            if ((parameterCount != 0 && (values == nullptr || present == nullptr || subOffsets == nullptr || subCounts == nullptr)) ||
                (subParameterCount != 0 && (subValues == nullptr || subPresent == nullptr)))
            {
                return false;
            }

            _parameters.clear();
            _subParameters.clear();
            _subParameterRanges.clear();
            _parameters.reserve(parameterCount);
            _subParameters.reserve(subParameterCount);
            _subParameterRanges.reserve(parameterCount);

            for (size_t i = 0; i < parameterCount; ++i)
            {
                _parameters.emplace_back(present[i] != 0 ? values[i] : -1);

                const auto offset = subOffsets[i];
                const auto count = subCounts[i];
                if (offset > subParameterCount || count > subParameterCount - offset)
                {
                    return false;
                }

                const auto end = offset + count;
                if (offset > std::numeric_limits<BYTE>::max() || end > std::numeric_limits<BYTE>::max())
                {
                    return false;
                }

                _subParameterRanges.emplace_back(static_cast<BYTE>(offset), static_cast<BYTE>(end));
            }

            for (size_t i = 0; i < subParameterCount; ++i)
            {
                _subParameters.emplace_back(subPresent[i] != 0 ? subValues[i] : -1);
            }

            const VTParameters parameters{
                std::span<const VTParameter>{ _parameters },
                std::span<const VTParameter>{ _subParameters },
                std::span<const std::pair<BYTE, BYTE>>{ _subParameterRanges }
            };
            return _engine.ActionCsiDispatch(VTID{ id }, parameters);
        }

        IStateMachineEngine& _engine;
        std::vector<VTParameter> _parameters;
        std::vector<VTParameter> _subParameters;
        std::vector<std::pair<BYTE, BYTE>> _subParameterRanges;
    };
}
