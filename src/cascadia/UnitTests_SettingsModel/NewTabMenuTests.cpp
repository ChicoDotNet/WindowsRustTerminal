// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#include "pch.h"

#include "../TerminalSettingsModel/NewTabMenuEntry.h"
#include "../TerminalSettingsModel/FolderEntry.h"
#include "../TerminalSettingsModel/CascadiaSettings.h"
#include "../TerminalSettingsModel/ActionMap.h"
#include "../TerminalSettingsModel/resource.h"
#include "../types/inc/colorTable.hpp"
#include "JsonTestClass.h"

using namespace Microsoft::Console;
using namespace winrt::Microsoft::Terminal;
using namespace winrt::Microsoft::Terminal::Settings::Model::implementation;
using namespace WEX::Logging;
using namespace WEX::TestExecution;
using namespace WEX::Common;

namespace SettingsModelUnitTests
{
    class NewTabMenuTests : public JsonTestClass
    {
        TEST_CLASS(NewTabMenuTests);

        TEST_METHOD(DefaultsToRemainingProfiles);
        TEST_METHOD(ParseEmptyFolder);
        TEST_METHOD(SuggestionsNestingContractReplay);
    };

    void NewTabMenuTests::DefaultsToRemainingProfiles()
    {
        Log::Comment(L"If the user doesn't customize the menu, put one entry for each profile");

        static constexpr std::string_view settingsString{ R"json({
        })json" };

        try
        {
            const auto settings{ winrt::make_self<CascadiaSettings>(settingsString, LoadStringResource(IDR_DEFAULTS)) };

            VERIFY_ARE_EQUAL(0u, settings->Warnings().Size());

            const auto& entries = settings->WindowSettingsDefaults().NewTabMenu();
            VERIFY_ARE_EQUAL(1u, entries.Size());
            VERIFY_ARE_EQUAL(winrt::Microsoft::Terminal::Settings::Model::NewTabMenuEntryType::RemainingProfiles, entries.GetAt(0).Type());
        }
        catch (const SettingsException& ex)
        {
            auto loadError = ex.Error();
            loadError;
            throw ex;
        }
        catch (const SettingsTypedDeserializationException& e)
        {
            auto deserializationErrorMessage = til::u8u16(e.what());
            Log::Comment(NoThrowString().Format(deserializationErrorMessage.c_str()));
            throw e;
        }
    }

    void NewTabMenuTests::ParseEmptyFolder()
    {
        Log::Comment(L"GH #14557 - An empty folder entry shouldn't crash");

        static constexpr std::string_view settingsString{ R"json({
            "newTabMenu": [
                { "type": "folder" }
            ]
        })json" };

        try
        {
            const auto settings{ winrt::make_self<CascadiaSettings>(settingsString, LoadStringResource(IDR_DEFAULTS)) };

            VERIFY_ARE_EQUAL(0u, settings->Warnings().Size());

            const auto& entries = settings->WindowSettingsDefaults().NewTabMenu();
            VERIFY_ARE_EQUAL(1u, entries.Size());
        }
        catch (const SettingsException& ex)
        {
            auto loadError = ex.Error();
            loadError;
            throw ex;
        }
        catch (const SettingsTypedDeserializationException& e)
        {
            auto deserializationErrorMessage = til::u8u16(e.what());
            Log::Comment(NoThrowString().Format(deserializationErrorMessage.c_str()));
            throw e;
        }
    }

    void NewTabMenuTests::SuggestionsNestingContractReplay()
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

        Log::Comment(L"Contract 1: enabled nesting preserves the parent command.");
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
            VERIFY_ARE_EQUAL(winrt::Microsoft::Terminal::Settings::Model::ShortcutAction::SendInput, command.ActionAndArgs().Action());
        }
    }
}
