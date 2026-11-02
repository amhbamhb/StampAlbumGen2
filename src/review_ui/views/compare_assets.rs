pub(super) fn render_compare_top_html(
    reconnect_banner_compare_html: &str,
    runtime_log_compare_panel_html: &str,
) -> String {
    format!(
        r#"<body>
<div class='wrap'>
    {reconnect_banner_compare_html}
    <div class='top'>
        <button class='neutral' onclick='returnToStartReviewPage()'>Back To Country Selection</button>
        <span class='badge'>Switch country without closing Review UI</span>
    </div>
    <div id='startGuide' class='startGuide'>
        <h2>Start Country Review Session</h2>
        <h2 ><strong>Current country:</strong> <span class='currentReviewCountry' style='color:#b45309;' >Unknown</span></h2>
        <p>Use the grouped controls to navigate candidates, assign matches, and apply updates safely without losing CLI/config capabilities.</p>
        <div class='guideStats'>
            <span id='statLeftCount'  class='badge'>Main Collection: 0</span>
            <span id='statRightCount'  class='badge'>Potential Candidates: 0</span>
            <span id='statAssignedCount' class='badge'>Assigned: 0</span>
        </div>
        <div class='guideActions'>
            <button class='primary guideStartReviewButton' onclick='dismissStartGuide()'>Start Reviewing</button>
            <button class='guideFieldOptionsButton' onclick='scrollToFilters()'>Review Field Options</button>
            <button class='guideComparePanelsButton' onclick='scrollToPanels()'>Jump To Compare Panels</button>
            <button class='neutral guideOpenCountryFolderButton' onclick='openCountryFolder()'>Open Country Folder</button>
            <button class='neutral reviewNavigationExitButton' onclick='closeReviewUi()'>Close Review UI and exit the program</button>
        </div>
  </div>
    {runtime_log_compare_panel_html}"#,
        reconnect_banner_compare_html = reconnect_banner_compare_html,
        runtime_log_compare_panel_html = runtime_log_compare_panel_html
    )
}

pub(super) fn render_compare_navigation_html() -> String {
    r#"        <div class='sessionSection sessionSectionSlate'>
                    <div class='sessionSectionTitle'>Navigation</div>
                    <div class='sessionRow'>
                        <button class='primary reviewNavigationBackButton' onclick='returnToStartReviewPage()'>Back To Country Selection</button>
                        <button class='neutral reviewNavigationExitButton' onclick='closeReviewUi()'>Close Review UI and exit the program</button>
                    </div>
                    <div id='status' class='sessionStatusBox sessionStatusSlate'>Ready</div>
                </div>"#.to_string()
}

pub(super) fn render_compare_filters_html() -> String {
    r#"        <div id='filtersCard' class='filtersCard'>
            <h4 class='filtersTitle'>Current Review Session for <span id='currentReviewSessionCountry'>Unknown</span> Options</h4>
            <div class='filtersGrid'>
        <label><input type='checkbox' id='f_series'/> series</label>
        <label><input type='checkbox' id='f_description' checked/> description</label>
        <label><input type='checkbox' id='f_watermark'/> watermark</label>
        <label><input type='checkbox' id='f_design'/> design</label>
        <label><input type='checkbox' id='f_engraving'/> engraving</label>
        <label><input type='checkbox' id='f_perforation'/> perforation</label>
        <label><input type='checkbox' id='f_context'/> context</label>
        <label><input type='checkbox' id='f_links' checked/> colnect links</label>
        <label><input type='checkbox' id='f_image_url' checked/> colnect image url</label>
        <label><input type='checkbox' id='f_colnect_meta' checked/> colnect metadata fields</label>
        <label><input type='checkbox' id='f_replace_sg'/> replace catalog number</label>
        <label><input type='checkbox' id='f_right_unmatched_only' onchange='render()'/> right unmatched only</label>
        <label><input type='checkbox' id='f_show_imageless_left' onchange='toggleImagelessLeftRows()'/> show left rows without images</label>
            <div class='orderSelectGroup'><label>order
                        <select id='sortMode' onchange='changeSortMode()'>
                        <option value='year_series_date_catalog' style='display:block;'>
                         Year then issue date in series then catalog number (default)</option>
                        <option value='catalog'>Catalog number</option>
                        <option value='year_catalog'>Year then catalog number</option>
                        <option value='series_catalog'>Series then catalog number</option>
                    <option value='series_date_catalog'style='display:block;'> Series then issue date then catalog number</option>
                                <option value='rowid'>DB row id</option>
                                <option value='colnect_id'>Colnect ID</option>
                        </select>
                </label><button id='sortModeMakeDefaultBtn' type='button' onclick='makeSortModeDefault()' style='display:none;'>Make default?</button>
                <button id='sortModeResetDefaultBtn' type='button' onclick='resetSortModeDefault()' style='display:none;'>Reset default</button></div>
      </div>
    </div>"#.to_string()
}

pub(super) fn render_compare_action_rows_html() -> String {
    r#"    <div class='actionRows'>


  
</div>"#
        .to_string()
}

pub(super) fn render_compare_panels_html() -> String {
    r#"  <div id='comparePanels' class='row'>
    <div class='col'>

        <div class='titleRow'>
            <h3>Left: StampWorld Base for <span class='currentReviewCountry' style='color:#b45309;'>Unknown</span></h3>
            <span id='leftPos' class='badge'></span>
        </div>
        <div id='leftActionBanner' class='leftActionBanner'></div>
        <div id='leftPanel'></div>
    </div>

    <div class='col'>
  
        <div class='actionRows'>
            <div class='actionBar candidate'>
                <span class='actionBarTitle'>Candidate Management</span>
                <button class='warn candidateManagementAddButton' onclick='openAddDialog()'>Add Colnect As
                    Stamp</button>
                <button class='danger candidateManagementRemoveButton' onclick='deleteCurrentCandidate()'>Delete
                    Candidate</button>
                <button class='candidateManagementUndoButton' onclick='undoLastDelete()'>Undo Delete</button>

            </div>
            <div class='actionBar match'>
                <span class='actionBarTitle'>Match Actions</span>
                <button class='success matchActionsAssignCanditateButton' onclick='assignCurrent()'>Assign
                    Candidate</button>
                <button class='matchActionsClearAssignmentButton' onclick='clearAssignment()'>Clear Assignment</button>
                <button class='primary matchActionsApplyMatchToDB' onclick='applyMatches()'>Apply Matches To DB</button>
            </div>
            <div id='matchActionStatus' class='actionMessage'>Ready</div>
            <div class='titleRow'>
                <h3>Right: Colnect Candidate for <span class='currentReviewCountry'
                        style='color:#b45309;'>Unknown</span></h3>
                <span id='rightPos' class='badge'></span>
            </div>
            <div id='rightActionBanner' class='rightActionBanner'></div>
            <div id='rightPanel'></div>
        </div>
    </div>"#.to_string()
}

pub(super) fn render_add_modal_html() -> String {
    r#"<div id='addModalBackdrop' class='modal-backdrop'>
    <div class='modal'>
        <h4>Add Colnect Candidate To stamps</h4>
        <div class='rowx'>
            <label><span id='addCatalogLabel'>Catalog</span> <input id='addSg' type='text' /></label>
            <label>Year <input id='addYear' type='number' /></label>
            <label>Series <input id='addSeries' type='text' /></label>
        </div>
        <div class='rowx'>
            <label><input id='addAssociateImage' type='checkbox' checked onchange='toggleAddImageOptions()'/> Associate image</label>
        </div>
        <div id='addImageOptions'>
            <div class='rowx'>
                <label><input type='radio' name='addImageMode' value='current_left' checked onchange='toggleAddImageOptions()'/> Use current left image</label>
                <label><input type='radio' name='addImageMode' value='upload' onchange='toggleAddImageOptions()'/> Upload image file</label>
                <label><input type='radio' name='addImageMode' value='none' onchange='toggleAddImageOptions()'/> No image</label>
            </div>
            <div class='rowx' id='uploadRow' style='display:none;'>
                <label>Upload <input id='addUploadFile' type='file' accept='image/png,image/jpeg,image/webp'/></label>
            </div>
        </div>
        <div class='rowx' style='justify-content:flex-end; margin-top: 12px;'>
            <button onclick='closeAddDialog()'>Cancel</button>
            <button onclick='addCurrentColnectAsStamp()'>Add</button>
        </div>
    </div>
</div>"#.to_string()
}

pub(super) fn render_op_busy_modal_html() -> String {
    r#"<div id='opBusyBackdrop' class='opBusyBackdrop'>
    <div class='opBusyCard'>
        <div class='opBusyRow'>
            <span class='opBusySpinner'></span>
            <div class='opBusyText'>
                <strong id='opBusyTitle'>Working<span id='opBusyDots' class='opBusyDots'></span></strong><br/>
                <span id='opBusyText'>Please wait...</span>
            </div>
        </div>
    </div>
</div>"#.to_string()
}

pub(super) fn compare_head_css() -> String {
    format!(
        "<style>{}{}{}</style>",
        include_str!("compare_assets.css"),
        include_str!("compare.css"),
        include_str!("country_url.css")
    )
}
