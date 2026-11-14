use super::{
    render_app_footer_html, render_config_editor_assets, render_footer_year_script,
    render_reconnect_banner_html, render_runtime_log_panel_html, MatcherPayload,
    BACKEND_NOT_RESPONDING_MESSAGE, FOOTER_YEAR_UPDATE_JS,
};

use super::compare_assets::{
    compare_head_css, render_add_modal_html, render_compare_action_rows_html,
    render_compare_filters_html, render_compare_navigation_html, render_compare_panels_html,
    render_compare_top_html, render_op_busy_modal_html,
};

use crate::review_ui::views::shared::ReconnectBannerParams;
use crate::review_ui::views::shared::RuntimeLogPanelParams;

pub(super) fn render_html(payload: &MatcherPayload) -> String {
    let json =
        serde_json::to_string(payload).unwrap_or_else(|_| "{\"left\":[],\"right\":[]}".to_string());
    let compare_css = compare_head_css();
    let reconnect_banner_compare_html = render_reconnect_banner_html(
        &(ReconnectBannerParams {
            banner_id: "reconnectBannerCompare",
            title_id: "reconnectBannerCompareTitle",
            text_id: None,
            text_html: None,
            extra_html: "\n        <button onclick='refreshState()'>Refresh</button>\n        <button onclick='openConfigEditor()'>Edit Config</button>",
            retry_fn: "retryBackendHealthCheckCompare",
            reload_fn: "reloadCurrentPageCompare",
            meta_id: "reconnectBannerCompareMeta",
        })
    );
    let runtime_log_compare_panel_html = render_runtime_log_panel_html(
        &(RuntimeLogPanelParams {
            panel_id: "runtimeLogComparePanel",
            panel_style: "border:1px solid #dbe4ec;border-radius:10px;background:#fbfdff;padding:10px;margin-bottom:10px;",
            header_style: "display:flex;gap:8px;justify-content:space-between;align-items:center;flex-wrap:wrap;margin-bottom:6px;",
            controls_style: "display:flex;gap:8px;align-items:center;flex-wrap:wrap;",
            body_id: "runtimeLogCompareBody",
            textarea_id: "runtimeLogCompare",
            checkbox_id: "showRuntimeLogCompare",
            toggle_fn: "toggleRuntimeLogVisibilityCompare",
            copy_fn: "copyRuntimeLogCompare",
        })
    );
    let compare_top_html = render_compare_top_html(
        &reconnect_banner_compare_html,
        &runtime_log_compare_panel_html,
    );
    let compare_navigation_html = render_compare_navigation_html();
    let compare_filters_html = render_compare_filters_html();
    let compare_action_rows_html = render_compare_action_rows_html();
    let compare_panels_html = render_compare_panels_html();
    let compare_add_modal_html = render_add_modal_html();
    let compare_op_busy_modal_html = render_op_busy_modal_html();
    let local_catalog_select_assets =
        " <div class='sessionRow'>
                        <button onclick='openConfigEditor()'>Edit Local Config</button>

                        <label for='defaultCatalogSelect'>Local Catalog</label>
                        <select id='defaultCatalogSelect' onchange='changeCountryDefaultCatalog()'>
                            <option value='sg'>Stanley Gibbons (SG)</option>
                                 <option value='sw'>StampWorld (SW)</option>
                            <option value='sn'>Scott Number (SN)</option>
                            <option value='mi'>Michel (MI)</option>
                            <option value='yt'>Yvert (YT)</option>
                            <option value='afa'>AFA</option>
                            <option value='nvp'>NVP</option>
                            <option value='un'>Unificato (UN)</option>
                            <option value='col'>Colnect (COL)</option>
                        </select>
                        <button onclick='saveCatalogDefaultToConfig()'>Update Local Config Default</button>
                    </div>";
    let import_colnect_files =
        " <div class='sessionSectionTitle'>Import Colnect - User Created Files</div>
        <div class='sessionRow'>
            <button id='importColnectPickBtn' onclick='pickAndCopyColnectFileReview()'>Browse &amp; Copy Colnect
                File&#8230;</button>
            <button id='importColnectBtn' onclick='importColnectFilesReview()'>Import Colnect - User Created Files</button>
            <button id='refreshColnectBtn' onclick='refreshColnectCandidatesReview()'>Refresh Colnect</button>
            <span id='reviewColnectCopyCountBadge' class='badge'>Files: 0</span>
        </div>";
    let colnect_destination_dir_path =
        "<div class='reviewColnectDestination'>
            Destination: <code id='reviewColnectDirPath' class='reviewColnectDirPath'>(unknown)</code>
        </div>";
    let colnect_import_status =
        " <div id='colnectImportStatus' class='colnectImportStatus sessionStatusBox sessionStatusGreen'>
            <div class='colnectImportSpinRow'>
                <span class='colnectImportSpinner initiallyHidden'></span>
                <span id='colnectImportStatusText' class='colnectImportStatusText'>Ready</span>
            </div>
            <div id='colnectImportLog' class='colnectImportLog initiallyHidden'></div>
        </div>";
    let stampworld_pull_year_setup =
        "
            <div class='sessionRow reviewSwRow reviewSwYearRow'>
                <div class='reviewSwYearLine'>
                    <label for='reviewSwStartYear' class='reviewPdfYearLabel'>Start Year</label>
                    <input id='reviewSwStartYear' type='number' placeholder='Start' min='1840' max='2100' />
                </div>
                <div class='reviewSwYearLine'>
                    <label for='reviewSwEndYear' class='reviewPdfYearLabel'>End Year</label>
                    <input id='reviewSwEndYear' type='number' placeholder='End' min='1840' max='2100' />
                </div>
            </div>";
    let stampworld_pull_command_preset = "
            <div class='sessionRow reviewSwRow reviewSwPresetRow'>
                <label for='reviewSwPreset'>Command Preset</label>
                <select id='reviewSwPreset'>
                    <option value='basic'>Basic Pull</option>
                    <option value='overwrite_album'>Overwrite Album Output</option>
                    <option value='overwrite_all'>Overwrite Album + Images</option>
                    <option value='bounded_pages'>Limit Pages (fast test)</option>
                    <option value='accept_typo'>Auto Accept Source Typos</option>
                </select>
            </div>";
    let stampworld_pull_command_info =
        "
           <div class='sessionRow reviewSwRow reviewSwFlagsRow'>
                        <label for='reviewSwExtraFlags'>Extra Flags</label>
                        <input id='reviewSwExtraFlags' class='reviewSwExtraFlagsInput' type='text' placeholder='--max-pages 20 --stop-after-no-hits 5' />
                    </div>
                    <div class='sessionRow reviewSwRow reviewSwActionRow'>
                        <button class='accent reviewSwRunButton' onclick='runReviewStampWorldPull()'>Run StampWorld pull for <span id='reviewSwRunCountry'>this country</span></button>
                    </div>
                    <div class='sessionRow reviewSwRow reviewSwCommandRow'>
                        <strong>Suggested command:</strong>
                        <code id='reviewSwCmd'></code>
                    </div>
                    <div class='sessionRow reviewSwRow reviewSwCommandRow'>
                        <button onclick='copyReviewStampWorldCommand()'>Copy</button>
                    </div>";
    let stampworld_pull_review_status =
        "
             <div id='reviewSwStatusWrap' class='reviewSwStatusWrap sessionStatusBox sessionStatusAmber'>
                        <span id='reviewSwRunSpinner' class='colnectImportSpinner initiallyHidden'></span>
                        <span id='reviewSwRunStatus' class='reviewSwStatusText'>Ready</span>
                    </div>
                    <div id='reviewSwPullLogPanel' class='reviewSwLogPanel initiallyHidden'>
                        <div class='reviewSwLogTitle'>StampWorld Pull Progress: Real-Time Log</div>
                        <textarea id='reviewSwPullLog' readonly class='reviewSwLogBox'></textarea>
                    </div>";

    let stampworld_sort_order_selector =
        " <div class='orderSelectGroup'>
            <label for='reviewPdfSortMode' class='reviewPdfSortLabel'>Order</label>
            <select id='reviewPdfSortMode' class='reviewPdfSortSelect' onchange='changePdfSortMode()'>
                <option value='year_series_date_catalog'>Year then issue date in series then catalog number (default)
                </option>
                <option value='catalog'>Catalog number</option>
                <option value='year_catalog'>Year then catalog number</option>
                <option value='series_catalog'>Series then catalog number</option>
                <option value='series_date_catalog'>Series then issue date then catalog number</option>
                <option value='rowid'>DB row id</option>
                <option value='colnect_id'>Colnect ID</option>
            </select>
            <button id='reviewPdfSortModeMakeDefaultBtn' class='initiallyHidden' type='button' onclick='makePdfSortModeDefault()'>Make default?</button>
            <button id='reviewPdfSortModeResetDefaultBtn' class='initiallyHidden' type='button' onclick='resetPdfSortModeDefault()'>Reset default</button>
        </div>";
    let stampworld_generate_pdf_button =
            "<button id='reviewPdfBtn' class='reviewPdfGenerateButton'
        onclick='generatePdfNowReview()' title='Generate the AlbumEasy PDF.'
        >Generate PDF Now</button>
            <button id='reviewPdfPreviewBtn' type='button' onclick='previewLastPdfReview()'>Preview Last PDF</button>";
    let stampworld_start_end_year =
        "<label for='reviewPdfYearRange' class='reviewPdfYearHint'>To Generate all pages, leave blank.</label>
        <div class='reviewPdfYearLine'>
            <label for='reviewPdfStartYear' class='reviewPdfYearLabel'>Start Year</label>
            <input id='reviewPdfStartYear' type='number' placeholder='Start Year' min='1840' max='2100'
                class='reviewPdfYearInput' />
        </div>
        <div class='reviewPdfYearLine'>
            <label for='reviewPdfEndYear' class='reviewPdfYearLabel'>End Year</label>
            <input id='reviewPdfEndYear' type='number' placeholder='End Year' min='1840' max='2100'
                class='reviewPdfYearInput' />
        </div>

        <div class='reviewPdfMetadataFilters'>
            <details class='reviewPdfFilter'>
                <summary id='reviewPdfemissionSummary'>Emission: All</summary>
                <div id='reviewPdfemissionChoices' class='reviewPdfFilterChoices' role='group' aria-label='PDF emission choices'></div>
            </details>
            <details class='reviewPdfFilter'>
                <summary id='reviewPdfformatSummary'>Format: All</summary>
                <div id='reviewPdfformatChoices' class='reviewPdfFilterChoices' role='group' aria-label='PDF format choices'></div>
            </details>
            <button id='reviewPdfRefreshFilters' type='button'>Refresh filter choices</button>
            <span id='reviewPdfFilterStatus' role='status' aria-live='polite'>Loading country filter choices...</span>
            <small>Emission/Format apply to Generate PDF Now only. Generate &amp; Close uses the year range, without these filters.</small>
        </div>

        <label for='reviewPdfAlwaysShow' class='reviewPdfAlwaysShowLabel'>

            <input id='reviewPdfAlwaysShow' type='checkbox' onchange='togglePdfAlwaysOpenPreference()' />Always open PDF after generation</label>";
    let stampworld_pdf_status =
        " <div id='pdfStatusWrap' class='pdfStatusWrap sessionStatusBox sessionStatusAmber'>
                        <span id='pdfStatusSpinner' class='colnectImportSpinner initiallyHidden'></span>
                        <span id='pdfStatus' class='pdfStatusText'>Ready</span>
                    </div>";
    // let left_edit_base_stamp_page = "<div class='leftEditorHead'>
    //         <h4 id='leftEditorTitle' class='leftEditorTitle'>123 Edit Base Stamp for <span id='currentReviewSessionCountry'>Unknown</span></h4>
    //         <p id='leftEditorHint' class='leftEditorHint'>Use this for StampWorld corrections, watermark/reference images, or fixing a bad Colnect placement.</p>
    //     </div>";
    let edit_based_page_fields = "<label>Catalog # / Ref Code <input id='leftEditorSg' type='text' /></label>
<label>Catalog Type Search <input id='leftEditorCatalogCodeSearch' type='text' list='leftEditorCatalogCodeSearchList'
        placeholder='Type to filter catalogs...' oninput='filterLeftEditorCatalogTypeOptions()'
        onchange='applyLeftEditorCatalogSearchChoice()' onblur='tryApplyLeftEditorCatalogSearchChoice()' /></label>
<label>Catalog Type <select id='leftEditorCatalogCodeSelect' onchange='toggleLeftEditorCatalogCodeCustom()'>
        <option value='sg'>Stanley Gibbons (SG)</option>
        <option value='sw'>StampWorld (SW)</option>
        <option value='sn'>Scott Number (SN)</option>
        <option value='mi'>Michel (MI)</option>
        <option value='yt'>Yvert (YT)</option>
        <option value='afa'>AFA</option>
        <option value='nvp'>NVP</option>
        <option value='un'>Unificato (UN)</option>
        <option value='col'>Colnect (COL)</option>
        <option value='custom'>Custom...</option>
    </select></label>
<label id='leftEditorCatalogCodeCustomWrap' style='display: none;'>Custom Catalog Type Code <input
        id='leftEditorCatalogCodeCustom' type='text' placeholder='e.g. zz' /></label>
<label>Year <input id='leftEditorYear' type='number' /></label>
<label>Series <input id='leftEditorSeries' type='text' /></label>
<label>Description <textarea id='leftEditorDescription'></textarea></label>
<label>Watermark <textarea id='leftEditorWatermark'></textarea></label>
<label>Design <textarea id='leftEditorDesign'></textarea></label>
<label>Engraving <textarea id='leftEditorEngraving'></textarea></label>
<label>Perforation <textarea id='leftEditorPerforation'></textarea></label>
<label>Context / Notes <textarea id='leftEditorContext'></textarea></label>";

    let edit_based_page_dimensions =" <div class='leftEditorSectionDimensions'>
    <div class='leftEditorDimsGrid' style='color: black;'>
        <label style='display: block;'>
            Width
            <input id='leftEditorWidth' type='text' placeholder='e.g. 25'
                style='display: block; width: 75%; margin-top: 0.25em;' />
        </label>

        <label style='display: block;'>
            Height
            <input id='leftEditorHeight' type='text' placeholder='e.g. 30'
                style='display: block; width: 75%; margin-top: 0.25em;' />
        </label>

        <label style='display: block;'>
            Colnect Width
            <input id='leftEditorColnectWidth' type='text' style='display: block; width: 75%; margin-top: 0.25em;' />
        </label>

        <label style='display: block;'>
            Colnect Height
            <input id='leftEditorColnectHeight' type='text' style='display: block; width: 75%; margin-top: 0.25em;' />
        </label>
    </div>
</div>";
    let compare_script_html = format!(
        "{}\n{}\n{}\n{}",
        include_str!("country_url.js"),
        include_str!("pdf_filters.js"),
        include_str!("stampworld_start_error.js"),
        include_str!("compare.js")
    )
    .replace("{json}", &json)
    .replace(
        "{backend_not_responding_message}",
        BACKEND_NOT_RESPONDING_MESSAGE,
    )
    .replace("{footer_year_update_js}", FOOTER_YEAR_UPDATE_JS);
    include_str!("compare.html")
        .replace("{country_url_controls}", include_str!("country_url.html"))
        .replace("{{", "{")
        .replace("}}", "}")
        .replace("{compare_css}", &compare_css)
        .replace("{compare_top_html}", &compare_top_html)
        .replace("{compare_script_html}", &compare_script_html)
        .replace("{local_catalog_select_assets}", local_catalog_select_assets)
        .replace("{import_colnect_files}", import_colnect_files)
        .replace(
            "{colnect_destination_dir_path}",
            colnect_destination_dir_path,
        )
        .replace("{colnect_import_status}", colnect_import_status)
        .replace("{stampworld_pull_year_setup}", stampworld_pull_year_setup)
        .replace(
            "{stampworld_pull_command_preset}",
            stampworld_pull_command_preset,
        )
        .replace(
            "{stampworld_pull_command_info}",
            stampworld_pull_command_info,
        )
        .replace(
            "{stampworld_pull_review_status}",
            stampworld_pull_review_status,
        )
        .replace(
            "{stampworld_generate_pdf_button}",
            stampworld_generate_pdf_button,
        )
        .replace(
            "{stampworld_sort_order_selector}",
            stampworld_sort_order_selector,
        )
        .replace("{stampworld_start_end_year}", stampworld_start_end_year)
        .replace("{stampworld_pdf_status}", stampworld_pdf_status)
        .replace("{compare_navigation_html}", &compare_navigation_html)
        .replace("{compare_filters_html}", &compare_filters_html)
        .replace("{compare_action_rows_html}", &compare_action_rows_html)
        .replace("{compare_panels_html}", &compare_panels_html)
        .replace("{compare_add_modal_html}", &compare_add_modal_html)
        .replace("{compare_op_busy_modal_html}", &compare_op_busy_modal_html)
        // .replace("{left_edit_base_stamp_page}", &left_edit_base_stamp_page)
        .replace("{edit_based_page_fields}", edit_based_page_fields)
        .replace("{edit_based_page_dimensions}", edit_based_page_dimensions)
        .replace("{footer_html}", &render_app_footer_html())
        .replace("{footer_year_script}", &render_footer_year_script())
        .replace("{config_editor_assets}", &render_config_editor_assets())
}
