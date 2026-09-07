# MUX / WinUI 2.8 upgrade archaeology

## `dev/migrie/mux-2.8-aug-2022`

**Disposition: NO-PORT / dependency upgrade experiment superseded.**

The only functional commit updates Microsoft.UI.Xaml (MUX/WinUI 2) to 2.8; the other exclusive commit is the mechanical spelling migration. Current Terminal uses Microsoft.UI.Xaml **2.8.4** with its current WebView2/additional-target integration, so this intermediate dependency-upgrade branch has no unique product contract.

## `dev/migrie/mux-2.8.2-march-2023`

**Disposition: NO-PORT / abandoned TabView compatibility investigation superseded by current WinUI.**

This branch tested MUX 2.8.2 and copied/customized TabView styles while investigating selected/manual tab color regressions. Its own commits record unresolved behavior and end with the explicit conclusion that the experiment should not proceed. The modern tree is on Microsoft.UI.Xaml 2.8.4 and no longer depends on this copied 2.8.2 workaround.

## Recovery guidance

If a future WinUI upgrade regresses tab colors, reproduce against the then-current WinUI/TabView templates; do not resurrect these version-specific 2.8/2.8.2 branches. The useful historical clue is that `SelectedBackgroundPath.Fill` in the 2.8.2 TabView template overrode custom tab backgrounds during this investigation.
