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
        <h2 ><strong>Current country:</strong> <span id='currentReviewCountry' style='color:#b45309;' >Unknown</span></h2>
        <p>Use the grouped controls to navigate candidates, assign matches, and apply updates safely without losing CLI/config capabilities.</p>
        <div class='guideStats'>
            <span id='statLeftCount' class='badge'>Left: 0</span>
            <span id='statRightCount' class='badge'>Candidates: 0</span>
            <span id='statAssignedCount' class='badge'>Assigned: 0</span>
        </div>
        <div class='guideActions'>
            <button class='primary' onclick='dismissStartGuide()'>Start Reviewing</button>
            <button onclick='scrollToFilters()'>Review Field Options</button>
            <button onclick='scrollToPanels()'>Jump To Compare Panels</button>
            <button class='neutral' onclick='openCountryFolder()'>Open Country Folder</button>
        </div>
  </div>
    {runtime_log_compare_panel_html}"#,
        reconnect_banner_compare_html = reconnect_banner_compare_html,
        runtime_log_compare_panel_html = runtime_log_compare_panel_html,
    )
}

pub(super) fn render_compare_navigation_html() -> String {
    r#"        <div class='sessionSection sessionSectionSlate'>
                    <div class='sessionSectionTitle'>Navigation</div>
                    <div class='sessionRow'>
                        <button onclick='returnToStartReviewPage()'>Back To Country Selection</button>
                        <button class='neutral' onclick='closeReviewUi()'>Close Review UI</button>
                    </div>
                    <div id='status' class='sessionStatusBox sessionStatusSlate'>Ready</div>
                </div>"#
        .to_string()
}

pub(super) fn render_compare_filters_html() -> String {
    r#"        <div id='filtersCard' class='filtersCard'>
            <h4 class='filtersTitle'>Field Update Options</h4>
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
                        <option value='year_series_date_catalog'>Year then issue date in series then catalog number (default)</option>
                        <option value='catalog'>Catalog number</option>
                        <option value='year_catalog'>Year then catalog number</option>
                        <option value='series_catalog'>Series then catalog number</option>
                    <option value='series_date_catalog'>Series then issue date then catalog number</option>
                                <option value='rowid'>DB row id</option>
                                <option value='colnect_id'>Colnect ID</option>
                        </select>
                </label><button id='sortModeMakeDefaultBtn' type='button' onclick='makeSortModeDefault()' style='display:none;'>Make default?</button>
                <button id='sortModeResetDefaultBtn' type='button' onclick='resetSortModeDefault()' style='display:none;'>Reset default</button></div>
      </div>
    </div>"#
        .to_string()
}

pub(super) fn render_compare_action_rows_html() -> String {
    r#"    <div class='actionRows'>
        <div class='actionBar match'>
            <span class='actionBarTitle'>Match Actions</span>
            <button class='success' onclick='assignCurrent()'>Assign Candidate</button>
            <button onclick='clearAssignment()'>Clear Assignment</button>
            <button class='primary' onclick='applyMatches()'>Apply Matches To DB</button>
        </div>
        <div class='actionBar candidate'>
            <span class='actionBarTitle'>Candidate Management</span>
            <button class='warn' onclick='openAddDialog()'>Add Colnect As Stamp</button>
            <button class='danger' onclick='deleteCurrentCandidate()'>Delete Candidate</button>
            <button onclick='undoLastDelete()'>Undo Delete</button>
        </div>
        <div id='matchActionStatus' class='actionMessage'>Ready</div>
    </div>"#
        .to_string()
}

pub(super) fn render_compare_panels_html() -> String {
        r#"  <div id='comparePanels' class='row'>
        <div class='col'>
                        <div class='titleRow'>
                                <h3>Left: StampWorld Base</h3>
                                <span id='leftPos' class='badge'></span>
                        </div>
                                                <div id='leftActionBanner' class='leftActionBanner'></div>
            <div id='leftPanel'></div>
        </div>
        <div class='col'>
                        <div class='titleRow'>
                                <h3>Right: Colnect Candidate</h3>
                                <span id='rightPos' class='badge'></span>
                        </div>
                                                <div id='rightActionBanner' class='rightActionBanner'></div>
            <div id='rightPanel'></div>
        </div>
    </div>"#
                .to_string()
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
</div>"#
        .to_string()
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
</div>"#
        .to_string()
}

pub(super) fn render_left_editor_modal_html() -> String {
    r#"<div id='leftEditorBackdrop' class='modal-backdrop'>
    <div class='modal leftEditorDialog leftEditorCompact'>
        <div class='leftEditorHead'>
            <h4 id='leftEditorTitle'>Edit Base Stamp</h4>
            <button onclick='closeLeftEditor()' class='closeButton'>&times;</button>
        </div>
        <div class='leftEditorBody'>
            <section class='leftEditorSection'>
                <h5>Base Stamp Details</h5>
                <div class='leftEditorSectionBody'>
                    <div class='leftEditorGrid'>
                        <label><span id='leftEditorCatalogLabel'>Catalog</span> <input id='leftEditorCatalogCode' type='text' list='catalogCodesList' placeholder='e.g. SG123' onchange='handleLeftEditorCatalogTypeChange()' />
                            <datalist id='catalogCodesList'>
                                <option value='sg'>Scott (SG)</option>
                                <option value='sn'>Scott Number (SN)</option>
                                <option value='mi'>Michel (MI)</option>
                                <option value='yt'>Yvert (YT)</option>
                                <option value='afa'>AFA</option>
                                <option value='nvp'>NVP</option>
                                <option value='un'>Unificato (UN)</option>
                                <option value='col'>Colnect (COL)</option>
                                <option value='custom'>Custom...</option>
                            </select>
                        </label>
                        <label id='leftEditorCatalogCodeCustomWrap' style='display:none;'>Custom Catalog Type Code <input id='leftEditorCatalogCodeCustom' type='text' placeholder='e.g. zz' /></label>
                        <label>Year <input id='leftEditorYear' type='number' /></label>
                        <label>Series <input id='leftEditorSeries' type='text' /></label>
                    </div>
                    <div class='leftEditorStack'>
                        <label>Description <textarea id='leftEditorDescription'></textarea></label>
                        <label>Watermark <textarea id='leftEditorWatermark'></textarea></label>
                        <label>Design <textarea id='leftEditorDesign'></textarea></label>
                        <label>Engraving <textarea id='leftEditorEngraving'></textarea></label>
                        <label>Perforation <textarea id='leftEditorPerforation'></textarea></label>
                        <label>Context / Notes <textarea id='leftEditorContext'></textarea></label>
                    </div>
                </div>
            </section>
            <section class='leftEditorSection'>
                <h5>Dimensions</h5>
                <div class='leftEditorSectionBody'>
                    <div class='leftEditorDimsGrid'>
                        <label>Width <input id='leftEditorWidth' type='text' placeholder='e.g. 25' /></label>
                        <label>Height <input id='leftEditorHeight' type='text' placeholder='e.g. 30' /></label>
                        <label>Colnect Width <input id='leftEditorColnectWidth' type='text' /></label>
                        <label>Colnect Height <input id='leftEditorColnectHeight' type='text' /></label>
                    </div>
                </div>
            </section>
            <section class='leftEditorSection'>
                <h5>Colnect Metadata</h5>
                <div class='leftEditorSectionBody'>
                    <div class='leftEditorGrid'>
                        <label>Colnect ID <input id='leftEditorColnectId' type='text' /></label>
                        <label>Colnect Name <input id='leftEditorColnectName' type='text' /></label>
                        <label>Colnect Country <input id='leftEditorColnectCountry' type='text' /></label>
                        <label>Catalog Codes <input id='leftEditorColnectCatalogCodes' type='text' /></label>
                        <label>Colnect Page URL <input id='leftEditorColnectPageUrl' type='text' /></label>
                        <label>Colnect Image URL <input id='leftEditorColnectImageUrl' type='text' /></label>
                        <label>Format <input id='leftEditorColnectFormat' type='text' /></label>
                        <label>Emission <input id='leftEditorColnectEmission' type='text' /></label>
                        <label>Printing <input id='leftEditorColnectPrinting' type='text' /></label>
                        <label>Paper <input id='leftEditorColnectPaper' type='text' /></label>
                        <label>Gum <input id='leftEditorColnectGum' type='text' /></label>
                        <label>Face Value <input id='leftEditorColnectFaceValue' type='text' /></label>
                        <label>Currency <input id='leftEditorColnectCurrency' type='text' /></label>
                        <label>Ord MI <input id='leftEditorColnectOrdMi' type='text' /></label>
                        <label>Ord SN <input id='leftEditorColnectOrdSn' type='text' /></label>
                        <label>Ord YT <input id='leftEditorColnectOrdYt' type='text' /></label>
                        <label>Ord Catalog (primary) <input id='leftEditorColnectOrdSg' type='text' /></label>
                        <label>Expiry Date <input id='leftEditorColnectExpiryDate' type='text' /></label>
                        <label>Print Run <input id='leftEditorColnectPrintRun' type='text' /></label>
                        <label>Colors <input id='leftEditorColnectColors' type='text' /></label>
                        <label>Parent Item <input id='leftEditorColnectParentItem' type='text' /></label>
                        <label>Variant <input id='leftEditorColnectVariant' type='text' /></label>
                        <label>Overprint Variant <input id='leftEditorColnectOverprintVariant' type='text' /></label>
                        <label>Score <input id='leftEditorColnectScore' type='text' /></label>
                        <label>Accuracy <input id='leftEditorColnectAccuracy' type='text' /></label>
                        <label>List <input id='leftEditorColnectList' type='text' /></label>
                        <label>Quantity <input id='leftEditorColnectQuantity' type='text' /></label>
                        <label>Condition <input id='leftEditorColnectCondition' type='text' /></label>
                    </div>
                    <div class='leftEditorStack'>
                        <label>Public Note <textarea id='leftEditorColnectPublicNote'></textarea></label>
                        <label>Private Note <textarea id='leftEditorColnectPrivateNote'></textarea></label>
                    </div>
                </div>
            </section>
            <section class='leftEditorSection'>
                <h5>Image Handling</h5>
                <div class='leftEditorSectionBody'>
                    <div class='leftEditorHint'>Items without an image will not stay visible in the left review list.</div>
                    <div class='leftEditorModes'>
                        <label id='leftModeKeepWrap'><input id='leftModeKeep' type='radio' name='leftImageMode' value='keep' checked onchange='toggleLeftEditorImageOptions()'/> Keep current image</label>
                        <label id='leftModeCurrentWrap'><input id='leftModeCurrent' type='radio' name='leftImageMode' value='current_left' onchange='toggleLeftEditorImageOptions()'/> Use current left image</label>
                        <label><input id='leftModeUpload' type='radio' name='leftImageMode' value='upload' onchange='toggleLeftEditorImageOptions()'/> Upload image</label>
                    </div>
                    <div id='leftUploadRow' class='rowx' style='display:none;'>
                        <label>Upload <input id='leftEditorUploadFile' type='file' accept='image/png,image/jpeg,image/webp'/></label>
                    </div>
                </div>
            </section>
        </div>
        <div class='leftEditorActions'>
            <button onclick='closeLeftEditor()'>Cancel</button>
            <button class='primary' onclick='submitLeftEditor()'>Save</button>
        </div>
    </div>
</div>"#
        .to_string()
}

pub(super) fn compare_head_css() -> String {
    format!(
        r#"<style>
body {{
  font-family: sans-serif;
  margin: 16px;
    background: linear-gradient(180deg, #eef4f8 0%, #f7f9fb 100%);
    opacity: 1;
    transition: opacity 170ms ease, filter 170ms ease, transform 170ms ease;
  --row-odd-bg: #ffffff;
  --row-even-bg: #e6eef4;
  --row-odd-key-bg: #f1f3f5;
  --row-even-key-bg: #d7e2eb;
    --accent-blue: #2f6fa3;
    --accent-green: #2f8f63;
    --accent-amber: #9b6a20;
    --accent-red: #a34545;
    --accent-slate: #4b5a68;
}}
body.pageLeaving {{ opacity: 0.2; filter: blur(1px); transform: translateY(2px); }}
body.pageEnter {{ animation: reviewPageIn 170ms ease-out; }}
@keyframes reviewPageIn {{ from {{ opacity: 0.45; transform: translateY(2px); }} to {{ opacity: 1; transform: translateY(0); }} }}
.wrap {{ background: #ffffff; border: 1px solid #dbe4ec; border-radius: 12px; padding: 14px; box-shadow: 0 8px 24px rgba(27, 51, 72, 0.08); }}
.top {{ display: flex; gap: 12px; align-items: center; flex-wrap: wrap; margin-bottom: 10px; }}
.startGuide {{ border: 1px solid #d8e3ed; border-left: 6px solid var(--accent-blue); border-radius: 10px; background: #f5fafe; padding: 12px; margin-bottom: 12px; }}
.startGuide h2 {{ margin: 0 0 4px 0; font-size: 18px; color: #173952; }}
.startGuide p {{ margin: 0; color: #35566d; font-size: 13px; }}
.guideStats {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-top: 10px; }}
.guideActions {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-top: 10px; }}
.controlGroups {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: 10px; margin-bottom: 10px; }}
.groupCard {{ border: 1px solid #dbe4ec; border-radius: 10px; background: #fbfdff; padding: 10px; }}
.groupCard h4 {{ margin: 0 0 8px 0; font-size: 13px; letter-spacing: 0.2px; }}
.groupCard.match {{ border-left: 5px solid var(--accent-green); }}
.groupCard.candidate {{ border-left: 5px solid var(--accent-amber); }}
.groupCard.session {{ border-left: 5px solid var(--accent-slate); }}
.groupBtns {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; }}
.sessionStack {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 8px; width: 100%; }}
.sessionSection {{ border-radius: 8px; padding: 8px; }}
.sessionSectionBlue {{ border: 1px solid #cddff2; background: #f2f8ff; }}
.sessionSectionGreen {{ border: 1px solid #cfe8da; background: #f2fbf6; }}
.sessionSectionAmber {{ border: 1px solid #ead8bc; background: #fff8ef; }}
.sessionSectionSlate {{ border: 1px solid #d5dbe3; background: #f5f6f8; }}
.sessionSectionTitle {{ margin: 0 0 6px 0; font-size: 12px; font-weight: 700; color: #2a4458; letter-spacing: 0.2px; }}
.sessionRow {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; }}
.reviewStampWorldSection {{ border: 1px solid #e3c27e; background: linear-gradient(180deg, #fff8ea 0%, #fffdf6 100%); }}
.reviewSwRow {{ display: flex; flex-direction: column; align-items: stretch; gap: 6px; padding: 10px 12px; border-radius: 10px; border: 1px solid transparent; }}
.reviewSwRow label {{ display: block; font-size: 12px; font-weight: 700; color: #31495d; }}
.reviewSwRow select,
.reviewSwRow input {{ width: 100%; max-width: 100%; min-width: 0; box-sizing: border-box; }}
.reviewSwYearRow {{ display: flex; flex-direction: row; align-items: center; flex-wrap: wrap; gap: 8px; }}
.reviewSwYearRow label {{ display: inline-block; margin-right: 2px; }}
.reviewSwYearRow input {{ width: 78px; padding: 2px 4px; font-size: 0.9em; }}
.reviewSwRow button {{ width: 100%; }}
.reviewSwCountryRow {{ background: #eef8ff; border-color: #c9dcee; }}
.reviewSwYearRow {{ background: #f3fff4; border-color: #cfe7d5; }}
.reviewSwPresetRow {{ background: #fff9ef; border-color: #ead9b6; }}
.reviewSwFlagsRow {{ background: #fff4f8; border-color: #e8c9d8; }}
.reviewSwActionRow {{ background: #eff2fb; border-color: #d4dbef; }}
.reviewSwCommandRow {{ background: #f7f3ff; border-color: #dfd2ee; }}
.reviewSwCommandRow code {{ display: block; width: 100%; white-space: pre-wrap; overflow-wrap: anywhere; word-break: break-word; }}
.reviewSwStatusWrap {{ display: flex; align-items: center; gap: 6px; padding: 6px 8px; min-height: 22px; }}
.reviewSwStatusText {{ font-size: 12px; color: inherit; font-weight: 600; }}
.reviewSwLogPanel {{ margin-top: 8px; border: 1px solid #e3c27e; border-radius: 6px; padding: 10px; background: #fffbf0; }}
.reviewSwLogTitle {{ font-size: 13px; font-weight: 600; color: #37566d; margin-bottom: 8px; }}
.reviewSwLogBox {{ width: 100%; min-height: 150px; max-height: 280px; resize: vertical; padding: 8px; border: 1px solid #d7b896; border-radius: 6px; font-family: monospace; font-size: 11px; background: #fffcf7; color: #333; overflow-y: auto; overflow-x: hidden; box-sizing: border-box; }}
.orderSelectGroup {{ display: inline-flex; gap: 8px; align-items: center; flex-wrap: wrap; min-width: 0; max-width: 100%; }}
.orderSelectGroup label {{ margin: 0; }}
#sortMode, #reviewPdfSortMode {{ min-width: 260px; width: min(560px, 100%); max-width: 100%; white-space: normal; }}
#sortMode option, #reviewPdfSortMode option {{ white-space: normal; }}
.sessionStatusBox {{ margin-top: 8px; min-height: 16px; font-size: 12px; border-radius: 8px; border-left-width: 4px; border-left-style: solid; padding: 6px 8px; }}
.sessionStatusBlue {{ border: 1px solid #cddff2; border-left: 4px solid #2f6fa3; background: #f8fcff; color: #2f4f67; }}
.sessionStatusGreen {{ border: 1px solid #cfe8da; border-left: 4px solid #2f8f63; background: #f4fcf8; color: #245641; }}
.sessionStatusAmber {{ border: 1px solid #ead8bc; border-left: 4px solid #c08a2b; background: #fffaf2; color: #694314; }}
.sessionStatusSlate {{ border: 1px solid #d5dbe3; border-left: 4px solid #5c6b78; background: #f7f8fa; color: #334b60; }}
.actionRows {{ display: flex; flex-direction: column; gap: 8px; margin-bottom: 10px; }}
.actionBar {{ border: 1px solid #dbe4ec; border-radius: 10px; padding: 9px 10px; background: #fbfdff; display: flex; gap: 10px; align-items: center; flex-wrap: wrap; }}
.actionBar.match {{ border-left: 5px solid var(--accent-green); }}
.actionBar.candidate {{ border-left: 5px solid var(--accent-amber); }}
.actionBarTitle {{ font-size: 12px; font-weight: 700; color: #2a4458; margin-right: 6px; }}
.actionMessage {{ border: 1px solid #dbe4ec; border-radius: 8px; background: #f7fbff; color: #35566d; padding: 7px 10px; font-size: 12px; min-height: 16px; }}
.filtersCard {{ border: 1px solid #dbe4ec; border-left: 5px solid var(--accent-blue); border-radius: 10px; background: #f8fbfe; padding: 10px; margin-bottom: 12px; }}
.filtersTitle {{ margin: 0 0 8px 0; font-size: 13px; color: #1f4663; }}
.filtersGrid {{ display: flex; gap: 10px; align-items: center; flex-wrap: wrap; }}
.leftActionBanner {{ border: 1px solid #e5dccd; border-left: 5px solid var(--accent-amber); border-radius: 10px; background: linear-gradient(180deg, #fffaf2 0%, #fffdf8 100%); padding: 10px; margin-bottom: 8px; }}
.leftActionTop {{ display: flex; gap: 10px; align-items: center; justify-content: space-between; flex-wrap: wrap; margin-bottom: 6px; }}
.leftActionTop strong {{ color: #5d4116; }}
.leftActionText {{ margin: 0; color: #6c5530; font-size: 12px; line-height: 1.45; }}
.leftActionBtns {{ display: flex; gap: 8px; flex-wrap: wrap; margin-top: 8px; }}
.leftActionNote {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-top: 6px; }}
.leftSearchRow {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-top: 8px; }}
.leftSearchRow input {{ width: 220px; max-width: 52vw; padding: 5px 7px; border: 1px solid #c8d5e2; border-radius: 7px; }}
.leftNavRow {{ display: flex; gap: 6px; align-items: center; flex-wrap: wrap; margin-top: 8px; background: #fff4e2; border: 1px solid #ead5b4; border-radius: 8px; padding: 6px; }}
.rightActionBanner {{ border: 1px solid #d6e4f2; border-left: 5px solid var(--accent-blue); border-radius: 10px; background: linear-gradient(180deg, #f4f9ff 0%, #f9fcff 100%); padding: 10px; margin-bottom: 8px; }}
.rightActionTop {{ display: flex; gap: 10px; align-items: center; justify-content: space-between; flex-wrap: wrap; margin-bottom: 6px; }}
.rightActionTop strong {{ color: #214d72; }}
.rightBottomBlock {{ display: flex; flex-direction: column; gap: 0; }}
.modal-backdrop {{
    display: none;
    position: fixed;
    inset: 0;
    background: rgba(17, 34, 49, 0.52);
    z-index: 1200;
    align-items: center;
    justify-content: center;
    padding: 16px;
    box-sizing: border-box;
}}
.modal {{
    width: min(860px, 96vw);
    max-height: 92vh;
    overflow: auto;
    background: #ffffff;
    border: 1px solid #dbe4ec;
    border-radius: 10px;
    box-shadow: 0 22px 52px rgba(17, 34, 49, 0.36);
    padding: 12px;
}}
.rowx {{ display: flex; gap: 8px; align-items: center; flex-wrap: wrap; margin-top: 8px; }}
.rowx label {{ display: inline-flex; align-items: center; gap: 6px; }}
.leftEditorDialog {{ width: min(1120px, 97vw); }}
.leftEditorCompact {{ max-height: 94vh; }}
.leftEditorHead {{ display: flex; align-items: center; justify-content: space-between; gap: 8px; margin-bottom: 8px; }}
.leftEditorHead h4 {{ margin: 0; }}
.closeButton {{ min-width: 36px; padding: 4px 8px; font-size: 18px; line-height: 1; }}
.leftEditorBody {{ display: flex; flex-direction: column; gap: 10px; }}
.leftEditorSection {{ border: 1px solid #dbe4ec; border-radius: 8px; background: #f9fcff; }}
.leftEditorSection h5 {{ margin: 0; padding: 8px 10px; border-bottom: 1px solid #dbe4ec; font-size: 13px; }}
.leftEditorSectionBody {{ padding: 10px; }}
.leftEditorGrid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 8px; }}
.leftEditorDimsGrid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 8px; }}
.leftEditorStack {{ display: grid; gap: 8px; margin-top: 8px; }}
.leftEditorStack label {{ display: grid; gap: 4px; }}
.leftEditorStack textarea {{ min-height: 56px; resize: vertical; }}
.leftEditorModes {{ display: flex; gap: 10px; align-items: center; flex-wrap: wrap; }}
.leftEditorActions {{ display: flex; justify-content: flex-end; gap: 8px; margin-top: 10px; }}
.opBusyBackdrop {{
    display: none;
    position: fixed;
    inset: 0;
    background: rgba(17, 34, 49, 0.5);
    z-index: 1300;
    align-items: center;
    justify-content: center;
    padding: 16px;
    box-sizing: border-box;
}}
.opBusyCard {{
    min-width: 280px;
    max-width: min(560px, 94vw);
    background: #ffffff;
    border: 1px solid #dbe4ec;
    border-radius: 10px;
    box-shadow: 0 18px 44px rgba(17, 34, 49, 0.34);
    padding: 12px;
}}
.opBusyRow {{ display: flex; align-items: center; gap: 10px; }}
.opBusySpinner {{
    width: 16px;
    height: 16px;
    border: 3px solid #d7e2eb;
    border-top-color: #2f6fa3;
    border-radius: 50%;
    animation: spin 0.9s linear infinite;
    flex: 0 0 auto;
}}
#comparePanels {{
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
    gap: 12px;
    align-items: start;
}}
#comparePanels .col {{ min-width: 0; }}
#comparePanels .col {{
    display: flex;
    flex-direction: column;
    min-height: 0;
}}
#leftPanel,
#rightPanel {{
    display: flex;
    min-width: 0;
    min-height: 0;
}}
#leftPanel > .panelCard,
#rightPanel > .panelCard {{
    width: 100%;
}}
.panelCard {{
    border: 1px solid #dbe4ec;
    border-radius: 10px;
    background: #ffffff;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    height: 74vh;
    min-height: 440px;
}}
.panelHead {{
    padding: 10px 12px;
    border-bottom: 1px solid #e1e8ef;
    background: #f7fbff;
}}
.panelBody {{
    padding: 10px;
    overflow-y: auto;
    min-height: 0;
    flex: 1 1 auto;
}}
.imgbox {{
    border: 1px solid #d9e2ea;
    border-radius: 8px;
    background: #f6f9fc;
    padding: 8px;
    margin-bottom: 8px;
    display: flex;
    justify-content: center;
    align-items: center;
}}
.imgbox img {{
    max-width: 100%;
    max-height: 220px;
    height: auto;
    object-fit: contain;
    display: block;
}}
.imgph {{
    color: #4b5a68;
    font-size: 12px;
    text-align: center;
}}
.kv {{
    width: 100%;
    border-collapse: separate;
    border-spacing: 0;
    table-layout: fixed;
}}
.kv td {{
    vertical-align: top;
    padding: 6px 8px;
    border-bottom: 1px solid #dde7ef;
    overflow-wrap: anywhere;
    word-break: break-word;
}}
.kv td.k {{
    width: 34%;
    font-weight: 600;
    color: #2f4f67;
    background: #f2f6fb;
}}
.kv td.v {{ background: #ffffff; }}
.opBusyText {{
    color: #1f3b52;
    font-size: 13px;
    line-height: 1.4;
}}
.opBusyDots {{
    display: inline-block;
    min-width: 18px;
    text-align: left;
}}
.reconnectBanner {{
    display: none;
    border: 1px solid #ddc79f;
    border-left: 5px solid #9b6a20;
    border-radius: 10px;
    background: #fff7e8;
    color: #694314;
    padding: 10px 12px;
    margin-bottom: 10px;
}}
.reconnectBanner.show {{
    display: block;
}}
.reconnectBanner.recovered {{
    border-color: #b9dec9;
    border-left-color: #2f8f63;
    background: #f4fcf8;
    color: #245641;
}}
.reconnectBanner .title {{
    font-weight: 700;
    margin-bottom: 4px;
}}
.reconnectBanner .meta {{
    font-size: 12px;
    color: #5e7283;
}}
.reconnectBanner .actions {{
    display: flex;
    gap: 8px;
    margin-top: 8px;
    flex-wrap: wrap;
}}
.colnectImportStatus {{ margin-top: 8px; }}
.colnectImportSpinRow {{ display: flex; align-items: center; gap: 8px; }}
.colnectImportSpinner {{ width: 14px; height: 14px; border: 2px solid #d7e2eb; border-top-color: #2f6fa3; border-radius: 50%; animation: colnectSpin 0.9s linear infinite; flex: 0 0 auto; display: inline-block; }}
.colnectImportStatusText {{ font-size: 12px; color: inherit; font-weight: 600; }}
.colnectImportLog {{ margin-top: 6px; max-height: 120px; overflow-y: auto; font-size: 11px; font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; background: #f2f7fb; border: 1px solid #d3dfe9; border-radius: 6px; padding: 4px 6px; }}
.colnectLogLine {{ padding: 2px 0; border-bottom: 1px solid #e8eef4; animation: colnectMsgIn 0.2s ease-out; }}
.colnectLogLine:last-child {{ border-bottom: none; }}
.reviewColnectDirPath {{ display: block; max-width: 100%; white-space: pre-wrap; overflow-wrap: anywhere; word-break: break-word; }}
.colnectImportDone {{ color: #2f8f63 !important; }}
.colnectImportFail {{ color: #a34545 !important; }}
.pdfStatusWrap {{
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 8px;
    min-height: 22px;
}}
.pdfStatusText {{
    font-size: 12px;
    color: inherit;
}}
.pdfStatusDone {{ color: #2f8f63; font-weight: 600; }}
.pdfStatusFail {{ color: #a34545; font-weight: 600; }}
@keyframes colnectSpin {{ from {{ transform: rotate(0deg); }} to {{ transform: rotate(360deg); }} }}
@keyframes colnectMsgIn {{ from {{ opacity: 0; transform: translateY(3px); }} to {{ opacity: 1; transform: translateY(0); }} }}
@keyframes spin {{ from {{ transform: rotate(0deg); }} to {{ transform: rotate(360deg); }} }}
@media (max-width: 980px) {{
    .row {{ flex-direction: column; }}
    #comparePanels {{ grid-template-columns: minmax(0, 1fr); }}
    .panelBody {{ max-height: 42vh; }}
    .orderSelectGroup {{ width: 100%; }}
    #sortMode, #reviewPdfSortMode {{ width: 100%; min-width: 190px; }}
}}
</style>"#
    )
}
