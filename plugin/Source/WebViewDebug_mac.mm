#include "WebViewDebug.h"

#import <Cocoa/Cocoa.h>
#import <WebKit/WebKit.h>

namespace
{
bool enableInspectableOnView (NSView* view)
{
    if (view == nil)
        return false;

    auto enabled = false;

    if ([view isKindOfClass: [WKWebView class]])
    {
        auto* webView = static_cast<WKWebView*> (view);

        if (@available (macOS 13.3, *))
        {
            webView.inspectable = YES;
            enabled = true;
        }

        [[webView configuration].preferences setValue: @(YES) forKey: @"developerExtrasEnabled"];
    }

    for (NSView* subview in [view subviews])
        enabled = enableInspectableOnView (subview) || enabled;

    return enabled;
}
}

namespace frostverb
{
bool enableWebViewInspection (juce::Component& root)
{
    auto enabled = false;

    if (auto* handle = root.getWindowHandle())
        enabled = enableInspectableOnView (static_cast<NSView*> (handle)) || enabled;

    for (auto i = 0; i < root.getNumChildComponents(); ++i)
        if (auto* child = root.getChildComponent (i))
            enabled = enableWebViewInspection (*child) || enabled;

    return enabled;
}
}
