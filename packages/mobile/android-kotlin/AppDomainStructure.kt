// The WebView page lives at https://dioxus.index.html (wry's custom protocol), and Chromium stamps
// that origin on the form and on every field it hands the autofill framework, so password managers
// saved logins under it. RustWebView (patch_autofill_domain in android.mk) passes this wrapper to
// Chromium instead, which reports the app's origin as counted.fr. ViewStructure is a write-only
// builder, so a wrapper is the only way to reach the field nodes; every method forwards, the
// non-abstract ones included, or the wrapper would silently drop what Chromium sets through them.
// Not the API 35 credential-manager ones: debug builds compile at SDK 34. Chromium sets those only
// on WebAuthn fields, which the app has none of; passkeys in the WebView would need them back.
package dev.dioxus.main

import android.graphics.Matrix
import android.net.Uri
import android.os.Bundle
import android.os.LocaleList
import android.view.ViewStructure
import android.view.autofill.AutofillId
import android.view.autofill.AutofillValue
import androidx.annotation.RequiresApi

private const val APP_HOST = "dioxus.index.html"
private const val SITE = "https://counted.fr"

@RequiresApi(26)
class AppDomainStructure(private val real: ViewStructure) : ViewStructure() {
    override fun setWebDomain(domain: String?) =
        real.setWebDomain(if (domain != null && (Uri.parse(domain).host ?: domain) == APP_HOST) SITE else domain)

    override fun newChild(index: Int): ViewStructure = AppDomainStructure(real.newChild(index))
    override fun asyncNewChild(index: Int): ViewStructure = AppDomainStructure(real.asyncNewChild(index))
    override fun asyncCommit() = real.asyncCommit()

    override fun addChildCount(num: Int): Int = real.addChildCount(num)
    override fun getAutofillId(): AutofillId? = real.autofillId
    override fun getChildCount(): Int = real.childCount
    override fun getExtras(): Bundle = real.extras
    override fun getHint(): CharSequence? = real.hint
    override fun getText(): CharSequence? = real.text
    override fun getTextSelectionEnd(): Int = real.textSelectionEnd
    override fun getTextSelectionStart(): Int = real.textSelectionStart
    override fun hasExtras(): Boolean = real.hasExtras()
    override fun newHtmlInfoBuilder(tagName: String): HtmlInfo.Builder = real.newHtmlInfoBuilder(tagName)
    override fun setAccessibilityFocused(state: Boolean) = real.setAccessibilityFocused(state)
    override fun setActivated(state: Boolean) = real.setActivated(state)
    override fun setAlpha(alpha: Float) = real.setAlpha(alpha)
    override fun setAutofillHints(hint: Array<out String>?) = real.setAutofillHints(hint)
    override fun setAutofillId(id: AutofillId) = real.setAutofillId(id)
    override fun setAutofillId(parentId: AutofillId, virtualId: Int) = real.setAutofillId(parentId, virtualId)
    override fun setAutofillOptions(options: Array<out CharSequence>?) = real.setAutofillOptions(options)
    override fun setAutofillType(type: Int) = real.setAutofillType(type)
    override fun setAutofillValue(value: AutofillValue?) = real.setAutofillValue(value)
    override fun setCheckable(state: Boolean) = real.setCheckable(state)
    override fun setChecked(state: Boolean) = real.setChecked(state)
    override fun setChildCount(num: Int) = real.setChildCount(num)
    override fun setClassName(className: String?) = real.setClassName(className)
    override fun setClickable(state: Boolean) = real.setClickable(state)
    override fun setContentDescription(contentDescription: CharSequence?) = real.setContentDescription(contentDescription)
    override fun setContextClickable(state: Boolean) = real.setContextClickable(state)
    override fun setDataIsSensitive(sensitive: Boolean) = real.setDataIsSensitive(sensitive)
    override fun setDimens(left: Int, top: Int, scrollX: Int, scrollY: Int, width: Int, height: Int) =
        real.setDimens(left, top, scrollX, scrollY, width, height)
    override fun setElevation(elevation: Float) = real.setElevation(elevation)
    override fun setEnabled(state: Boolean) = real.setEnabled(state)
    override fun setFocusable(state: Boolean) = real.setFocusable(state)
    override fun setFocused(state: Boolean) = real.setFocused(state)
    override fun setHint(hint: CharSequence?) = real.setHint(hint)
    override fun setHtmlInfo(htmlInfo: HtmlInfo) = real.setHtmlInfo(htmlInfo)
    override fun setId(id: Int, packageName: String?, typeName: String?, entryName: String?) =
        real.setId(id, packageName, typeName, entryName)
    override fun setInputType(inputType: Int) = real.setInputType(inputType)
    override fun setLocaleList(localeList: LocaleList?) = real.setLocaleList(localeList)
    override fun setLongClickable(state: Boolean) = real.setLongClickable(state)
    override fun setOpaque(opaque: Boolean) = real.setOpaque(opaque)
    override fun setSelected(state: Boolean) = real.setSelected(state)
    override fun setText(text: CharSequence?) = real.setText(text)
    override fun setText(text: CharSequence?, selectionStart: Int, selectionEnd: Int) =
        real.setText(text, selectionStart, selectionEnd)
    override fun setTextLines(charOffsets: IntArray?, baselines: IntArray?) = real.setTextLines(charOffsets, baselines)
    override fun setTextStyle(size: Float, fgColor: Int, bgColor: Int, style: Int) =
        real.setTextStyle(size, fgColor, bgColor, style)
    override fun setTransformation(matrix: Matrix?) = real.setTransformation(matrix)
    override fun setVisibility(visibility: Int) = real.setVisibility(visibility)

    @RequiresApi(28)
    override fun setImportantForAutofill(mode: Int) = real.setImportantForAutofill(mode)
    @RequiresApi(28)
    override fun setMaxTextEms(maxEms: Int) = real.setMaxTextEms(maxEms)
    @RequiresApi(28)
    override fun setMaxTextLength(maxLength: Int) = real.setMaxTextLength(maxLength)
    @RequiresApi(28)
    override fun setMinTextEms(minEms: Int) = real.setMinTextEms(minEms)
    @RequiresApi(30)
    override fun setHintIdEntry(entryName: String) = real.setHintIdEntry(entryName)
    @RequiresApi(30)
    override fun setTextIdEntry(entryName: String) = real.setTextIdEntry(entryName)
    @RequiresApi(31)
    override fun setReceiveContentMimeTypes(mimeTypes: Array<out String>?) = real.setReceiveContentMimeTypes(mimeTypes)
}
