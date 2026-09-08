from pathlib import Path


def read_preserving(path):
    p = Path(path)
    text = p.read_bytes().decode("utf-8")
    eol = "\r\n" if "\r\n" in text else "\n"
    return p, text, eol


def write_preserving(path, text):
    Path(path).write_bytes(text.encode("utf-8"))


def replace_once(path, old, new):
    p, text, eol = read_preserving(path)
    old = old.replace("\n", eol)
    new = new.replace("\n", eol)
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{path}: expected exactly one occurrence, found {count}: {old[:100]!r}")
    write_preserving(p, text.replace(old, new, 1))


# ActionArgs.idl: recover the user-facing nesting option.
replace_once(
    "src/cascadia/TerminalSettingsModel/ActionArgs.idl",
    "    interface INewContentArgs {",
    """    enum SuggestionsNesting
    {
        Disabled,
        Enabled,
    };

    interface INewContentArgs {""",
)
replace_once(
    "src/cascadia/TerminalSettingsModel/ActionArgs.idl",
    """        SuggestionsArgs(SuggestionsSource source, Boolean useCommandline);
        SuggestionsSource Source { get; };
        Boolean UseCommandline { get; };""",
    """        SuggestionsArgs(SuggestionsSource source, SuggestionsNesting nesting, Boolean useCommandline);
        SuggestionsSource Source { get; };
        SuggestionsNesting Nesting { get; };
        Boolean UseCommandline { get; };""",
)

# ActionArgs.h: insert the x-macro row while preserving the file's line endings.
args_h, text, eol = read_preserving("src/cascadia/TerminalSettingsModel/ActionArgs.h")
lines = text.splitlines(keepends=True)
source_marker = '    X(SuggestionsSource, Source, "source", false, ArgTypeHint::None, SuggestionsSource::Tasks)'
bool_marker = '    X(bool, UseCommandline, "useCommandline", false, ArgTypeHint::None, false)'
matches = [i for i, line in enumerate(lines) if source_marker in line]
if len(matches) != 1:
    raise SystemExit(f"ActionArgs.h: expected one SuggestionsSource row, found {len(matches)}")
i = matches[0]
if i + 1 >= len(lines) or bool_marker not in lines[i + 1]:
    raise SystemExit("ActionArgs.h: Suggestions macro layout drifted")
lines.insert(
    i + 1,
    '    X(SuggestionsNesting, Nesting, "nesting", false, ArgTypeHint::None, SuggestionsNesting::Enabled) \\' + eol,
)
write_preserving(args_h, "".join(lines))

# WinRT ActionMap contract: nesting is explicit at the query boundary.
replace_once(
    "src/cascadia/TerminalSettingsModel/ActionMap.h",
    "        winrt::Windows::Foundation::IAsyncOperation<winrt::Windows::Foundation::Collections::IVector<Model::Command>> FilterToSnippets(winrt::hstring currentCommandline, winrt::hstring currentWorkingDirectory);",
    "        winrt::Windows::Foundation::IAsyncOperation<winrt::Windows::Foundation::Collections::IVector<Model::Command>> FilterToSnippets(winrt::hstring currentCommandline, winrt::hstring currentWorkingDirectory, Model::SuggestionsNesting nesting);",
)
replace_once(
    "src/cascadia/TerminalSettingsModel/ActionMap.idl",
    "        Windows.Foundation.IAsyncOperation<IVector<Command> > FilterToSnippets(String CurrentCommandline, String CurrentWorkingDirectory);",
    "        Windows.Foundation.IAsyncOperation<IVector<Command> > FilterToSnippets(String CurrentCommandline, String CurrentWorkingDirectory, SuggestionsNesting nesting);",
)

# JSON enum mapping used by showSuggestions action deserialization.
replace_once(
    "src/cascadia/TerminalSettingsModel/TerminalSettingsSerializationHelpers.h",
    "JSON_ENUM_MAPPER(::winrt::Microsoft::Terminal::Settings::Model::WindowingMode)",
    """JSON_ENUM_MAPPER(::winrt::Microsoft::Terminal::Settings::Model::SuggestionsNesting)
{
    JSON_MAPPINGS(2) = {
        pair_type{ "enabled", ValueType::Enabled },
        pair_type{ "disabled", ValueType::Disabled },
    };
};

JSON_ENUM_MAPPER(::winrt::Microsoft::Terminal::Settings::Model::WindowingMode)""",
)

# Transform the old synchronous sendInput filter contract into today's async snippets pipeline.
replace_once(
    "src/cascadia/TerminalSettingsModel/ActionMap.cpp",
    """    std::vector<Model::Command> _filterToSnippets(IMapView<hstring, Model::Command> nameMap,
                                                  winrt::hstring currentCommandline,
                                                  const std::vector<Model::Command>& localCommands)
""",
    """    std::vector<Model::Command> _filterToSnippets(IMapView<hstring, Model::Command> nameMap,
                                                  winrt::hstring currentCommandline,
                                                  Model::SuggestionsNesting nesting,
                                                  const std::vector<Model::Command>& localCommands)
""",
)
replace_once(
    "src/cascadia/TerminalSettingsModel/ActionMap.cpp",
    """                auto innerResults = winrt::single_threaded_vector<Model::Command>(_filterToSnippets(command.NestedCommands(), currentCommandline, empty));

                if (innerResults.Size() > 0)
                {
                    // This command did have at least one sendInput under it

                    // Create a new Command, which is a copy of this Command,
                    // which only has SendInputs in it
                    winrt::com_ptr<implementation::Command> cmdImpl;
                    cmdImpl.copy_from(winrt::get_self<implementation::Command>(command));
                    auto copy = cmdImpl->Copy();
                    copy->NestedCommands(innerResults.GetView());

                    results.push_back(*copy);
                }
""",
    """                auto innerResults = winrt::single_threaded_vector<Model::Command>(_filterToSnippets(command.NestedCommands(), currentCommandline, nesting, empty));

                if (innerResults.Size() > 0)
                {
                    // This command did have at least one sendInput under it.
                    if (nesting == Model::SuggestionsNesting::Enabled)
                    {
                        // Preserve the parent and its hierarchy.
                        winrt::com_ptr<implementation::Command> cmdImpl;
                        cmdImpl.copy_from(winrt::get_self<implementation::Command>(command));
                        auto copy = cmdImpl->Copy();
                        copy->NestedCommands(innerResults.GetView());

                        results.push_back(*copy);
                    }
                    else if (nesting == Model::SuggestionsNesting::Disabled)
                    {
                        // Flatten the recursively-filtered leaves into this level.
                        for (const auto& nested : innerResults)
                        {
                            results.push_back(nested);
                        }
                    }
                }
""",
)
replace_once(
    "src/cascadia/TerminalSettingsModel/ActionMap.cpp",
    """    winrt::Windows::Foundation::IAsyncOperation<IVector<Model::Command>> ActionMap::FilterToSnippets(
        winrt::hstring currentCommandline,
        winrt::hstring currentWorkingDirectory)
""",
    """    winrt::Windows::Foundation::IAsyncOperation<IVector<Model::Command>> ActionMap::FilterToSnippets(
        winrt::hstring currentCommandline,
        winrt::hstring currentWorkingDirectory,
        Model::SuggestionsNesting nesting)
""",
)
replace_once(
    "src/cascadia/TerminalSettingsModel/ActionMap.cpp",
    """_filterToSnippets(NameMap(),
                                                                                          currentCommandline,
                                                                                          localSnippets)""",
    """_filterToSnippets(NameMap(),
                                                                                          currentCommandline,
                                                                                          nesting,
                                                                                          localSnippets)""",
)
replace_once(
    "src/cascadia/TerminalSettingsModel/ActionMap.cpp",
    """_filterToSnippets(NameMap(),
                                                                                  currentCommandline,
                                                                                  localSnippets)""",
    """_filterToSnippets(NameMap(),
                                                                                  currentCommandline,
                                                                                  nesting,
                                                                                  localSnippets)""",
)

# Wire showSuggestions through the recovered argument.
replace_once(
    "src/cascadia/TerminalApp/AppActionHandlers.cpp",
    "            const auto tasks = co_await _settings.GlobalSettings().ActionMap().FilterToSnippets(currentCommandline, currentWorkingDirectory);",
    "            const auto tasks = co_await _settings.GlobalSettings().ActionMap().FilterToSnippets(currentCommandline, currentWorkingDirectory, realArgs.Nesting());",
)

# The dedicated snippets pane intentionally retains its hierarchical tree.
replace_once(
    "src/cascadia/TerminalApp/SnippetsPaneContent.cpp",
    "        const auto tasks = co_await _settings.GlobalSettings().ActionMap().FilterToSnippets(winrt::hstring{}, winrt::hstring{}); // IVector<Model::Command>",
    "        const auto tasks = co_await _settings.GlobalSettings().ActionMap().FilterToSnippets(winrt::hstring{}, winrt::hstring{}, SuggestionsNesting::Enabled); // IVector<Model::Command>",
)
