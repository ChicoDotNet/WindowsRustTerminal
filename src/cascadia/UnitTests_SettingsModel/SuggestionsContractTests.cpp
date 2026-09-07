// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#include "pch.h"

#include "../TerminalSettingsModel/ActionMap.h"
#include "JsonTestClass.h"

using namespace WEX::Logging;
using namespace WEX::TestExecution;
using namespace WEX::Common;
using namespace winrt::Microsoft::Terminal::Settings::Model;
using namespace winrt::Microsoft::Terminal::Settings::Model::implementation;

namespace SettingsModelUnitTests
{
    class SuggestionsContractTests : public JsonTestClass
    {
        TEST_CLASS(SuggestionsContractTests);

        TEST_METHOD(NestingContractReplay);
    };

    void SuggestionsContractTests::NestingContractReplay()
    {
        static constexpr std::string_view settingsString{ R"json({
            "actions": [
                {
                    "name": "Git...",
                    "commands": [
                        { "name": "Commit", "command": { "action": "sendInput", "input": "git commit" } },
                        { "name": "Status", "command": { "action": "sendInput", "input": "git status" } }
                    ]
                }
            ],
            "keybindings": []
        })json" };

        auto actionMap{ ActionMap::FromJson(VerifyParseSucceeded(settingsString)) };
        actionMap->_FinalizeInheritance();

        Log::Comment(L"Contract 1: default/enabled nesting preserves the parent command.");
        const auto nested{ actionMap->FilterToSnippets(L"", L"", SuggestionsNesting::Enabled).get() };
        VERIFY_ARE_EQUAL(1u, nested.Size());
        VERIFY_IS_TRUE(nested.GetAt(0).HasNestedCommands());
        VERIFY_ARE_EQUAL(2u, nested.GetAt(0).NestedCommands().Size());

        Log::Comment(L"Contract 2: disabled nesting returns the same leaves at top level.");
        const auto flat{ actionMap->FilterToSnippets(L"", L"", SuggestionsNesting::Disabled).get() };
        VERIFY_ARE_EQUAL(2u, flat.Size());
        for (const auto& command : flat)
        {
            VERIFY_IS_FALSE(command.HasNestedCommands());
            VERIFY_ARE_EQUAL(ShortcutAction::SendInput, command.ActionAndArgs().Action());
        }
    }
}
