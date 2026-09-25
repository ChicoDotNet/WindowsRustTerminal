// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#include "precomp.h"
#include "screenInfo.hpp"

#include "output.h"
#include "../interactivity/inc/ServiceLocator.hpp"
#include "../types/inc/CodepointWidthDetector.hpp"
#include "../terminal/adapter/adaptDispatch.hpp"
#include "../terminal/parser/OutputStateMachineEngine.hpp"
#include "../terminal/parser/stateMachine.hpp"

using namespace Microsoft::Console;
using namespace Microsoft::Console::Types;
using namespace Microsoft::Console::Render;
using namespace Microsoft::Console::Interactivity;
using namespace Microsoft::Console::VirtualTerminal;

#pragma region Construct_Destruct

SCREEN_INFORMATION::SCREEN_INFORMATION(
    _In_ IWindowMetrics* pMetrics,
    const TextAttribute popupAttributes,
    const FontInfo fontInfo) :
    _pConsoleWindowMetrics{ pMetrics },
    _PopupAttributes{ popupAttributes },
    _currentFont{ fontInfo },
    _desiredFont{ fontInfo }
{
    // Check if VT mode should be enabled by default. This can be true if
    // VirtualTerminalLevel is set to !=0 in the registry, or when conhost
    // is started in conpty mode.
    const auto& gci = ServiceLocator::LocateGlobals().getConsoleInformation();
    if (gci.GetDefaultVirtTermLevel() != 0)
    {
        OutputMode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
    }
    _desiredFont.SetEnableBuiltinGlyphs(gci.GetEnableBuiltinGlyphs());
}
