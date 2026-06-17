```

warning: `codex-analytics` (lib) generated 157 warnings (run `cargo fix --lib -p codex-analytics` to apply 3 suggestions)
   Compiling codex-tui v0.0.0 (C:\CodexLab\codex\codex-rs\tui)
   Compiling codex-cloud-tasks v0.0.0 (C:\CodexLab\codex\codex-rs\cloud-tasks)
   Compiling codex-cli v0.0.0 (C:\CodexLab\codex\codex-rs\cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1m 50s
------------
 Nextest run ID 2f0823d5-bed0-42f5-b7bc-9fe4668330e4 with nextest profile: default
    Starting 2796 tests across 6 binaries (9 tests skipped)
        PASS [   0.030s] codex-tui additional_dirs::tests::returns_none_for_external_sandbox
        PASS [   0.034s] codex-tui additional_dirs::tests::returns_none_when_no_additional_dirs
        PASS [   0.045s] codex-tui additional_dirs::tests::warns_when_profile_can_write_elsewhere_but_not_cwd
        PASS [   0.048s] codex-tui additional_dirs::tests::returns_none_for_workspace_write
        PASS [   0.053s] codex-tui additional_dirs::tests::warns_for_read_only
        PASS [   0.056s] codex-tui app::agent_navigation::tests::adjacent_thread_id_wraps_in_spawn_order
        PASS [   0.063s] codex-tui additional_dirs::tests::returns_none_for_danger_full_access
        PASS [   0.066s] codex-tui app::agent_navigation::tests::active_agent_label_tracks_current_thread
        PASS [   0.075s] codex-tui app::app_server_event_targets::tests::thread_settings_updated_notifications_route_to_threads
        PASS [   0.080s] codex-tui app::app_server_event_targets::tests::guardian_warning_notifications_route_to_threads
        PASS [   0.084s] codex-tui app::agent_navigation::tests::upsert_preserves_first_seen_order
        PASS [   0.091s] codex-tui app::agent_navigation::tests::picker_subtitle_mentions_shortcuts
        PASS [   0.069s] codex-tui app::app_server_event_targets::tests::warning_notifications_route_to_threads_when_thread_id_is_present
        PASS [   0.070s] codex-tui app::app_server_event_targets::tests::warning_notifications_without_threads_are_global
        PASS [   0.065s] codex-tui app::app_server_requests::tests::correlates_mcp_elicitation_server_request_with_resolution
        PASS [   0.068s] codex-tui app::app_server_requests::tests::does_not_mark_chatgpt_auth_refresh_as_unsupported
        PASS [   0.062s] codex-tui app::app_server_requests::tests::rejects_dynamic_tool_calls_as_unsupported
        PASS [   0.068s] codex-tui app::app_server_requests::tests::resolve_notification_returns_resolved_exec_request
        PASS [   0.070s] codex-tui app::app_server_requests::tests::resolve_notification_returns_resolved_mcp_request
        PASS [   0.068s] codex-tui app::app_server_requests::tests::resolve_notification_returns_resolved_user_input_item_id
        PASS [   0.067s] codex-tui app::app_server_requests::tests::resolves_exec_approval_through_app_server_request_id
        PASS [   0.063s] codex-tui app::app_server_requests::tests::resolves_patch_approval_through_app_server_request_id
        PASS [   0.069s] codex-tui app::app_server_requests::tests::resolves_permissions_and_user_input_through_app_server_request_id
        PASS [   0.064s] codex-tui app::app_server_requests::tests::same_turn_user_input_answers_resolve_app_server_requests_fifo
        PASS [   0.065s] codex-tui app::background_requests::tests::build_feedback_upload_params_includes_thread_id_and_rollout_path
        PASS [   0.063s] codex-tui app::background_requests::tests::build_feedback_upload_params_omits_rollout_path_without_logs
        PASS [   0.066s] codex-tui app::background_requests::tests::hide_cli_only_plugin_marketplaces_removes_openai_bundled
        PASS [   0.066s] codex-tui app::background_requests::tests::marketplace_add_source_for_request_resolves_relative_local_paths
        PASS [   0.069s] codex-tui app::background_requests::tests::mcp_inventory_maps_prefix_tool_names_by_server
        PASS [   0.112s] codex-tui app::background_requests::tests::mcp_inventory_omits_thread_id_for_closed_agent_thread
        PASS [   0.117s] codex-tui app::config_persistence::tests::overridden_disabled_guardian_does_not_apply_auto_review_companions
        PASS [   0.137s] codex-tui app::config_persistence::tests::rebuild_config_for_resume_or_fallback_errors_when_cwd_changes
        PASS [   0.135s] codex-tui app::config_persistence::tests::rebuild_config_for_resume_or_fallback_uses_current_config_on_same_cwd_error
        PASS [   0.139s] codex-tui app::config_persistence::tests::refresh_in_memory_config_from_disk_best_effort_keeps_current_config_on_error
        PASS [   0.139s] codex-tui app::config_persistence::tests::sync_tui_pet_disabled_updates_chat_widget_config_copy
        PASS [   0.132s] codex-tui app::config_persistence::tests::sync_tui_pet_selection_updates_chat_widget_config_copy
        PASS [   0.153s] codex-tui app::config_persistence::tests::refresh_in_memory_config_from_disk_loads_latest_apps_state
        PASS [   0.159s] codex-tui app::config_persistence::tests::refresh_in_memory_config_from_disk_updates_resize_reflow_config
        PASS [   0.160s] codex-tui app::config_persistence::tests::refresh_in_memory_config_from_disk_uses_active_chat_widget_cwd
        PASS [   0.142s] codex-tui app::config_persistence::tests::sync_tui_theme_selection_updates_chat_widget_config_copy
        PASS [   0.147s] codex-tui app::config_persistence::tests::update_reasoning_effort_updates_collaboration_mode
        PASS [   0.058s] codex-tui app::pending_interactive_replay::tests::request_user_input_does_not_count_as_pending_thread_approval
        PASS [   0.044s] codex-tui app::pending_interactive_replay::tests::thread_event_snapshot_drops_answered_request_user_input_for_multi_prompt_turn
        PASS [   0.066s] codex-tui app::loaded_threads::tests::finds_loaded_subagent_tree_for_primary_thread
        PASS [   0.057s] codex-tui app::pending_interactive_replay::tests::thread_event_snapshot_drops_pending_approvals_when_turn_completes
        PASS [   0.061s] codex-tui app::pending_interactive_replay::tests::thread_event_snapshot_drops_pending_requests_when_thread_closes
        PASS [   0.113s] codex-tui app::input::tests::app_keymap_shortcuts_are_disabled_while_keymap_view_is_active
        PASS [   0.067s] codex-tui app::pending_interactive_replay::tests::thread_event_snapshot_drops_resolved_exec_approval_after_outbound_approval_id
        PASS [   0.073s] codex-tui app::pending_interactive_replay::tests::thread_event_snapshot_drops_resolved_elicitation_after_outbound_resolution
        PASS [   0.066s] codex-tui app::pending_interactive_replay::tests::thread_event_snapshot_drops_resolved_exec_approval_after_server_resolution
        PASS [   0.040s] codex-tui app::pending_interactive_replay::tests::thread_event_snapshot_drops_resolved_patch_approval_after_outbound_approval
        PASS [   0.050s] codex-tui app::pending_interactive_replay::tests::thread_event_snapshot_drops_resolved_request_user_input_after_user_answer
        PASS [   0.056s] codex-tui app::pending_interactive_replay::tests::thread_event_snapshot_drops_resolved_request_user_input_after_server_resolution
        PASS [   0.066s] codex-tui app::pending_interactive_replay::tests::thread_event_snapshot_keeps_newer_request_user_input_pending_when_same_turn_has_queue
        PASS [   0.067s] codex-tui app::pending_interactive_replay::tests::thread_event_store_reports_pending_thread_approvals
        PASS [   0.075s] codex-tui app::pending_interactive_replay::tests::thread_event_snapshot_keeps_pending_request_user_input
        PASS [   0.070s] codex-tui app::platform_actions::tests::side_return_shortcuts_match_ctrl_c_and_ctrl_d
        PASS [   0.074s] codex-tui app::session_lifecycle::tests::closed_state_for_thread_read_error_marks_terminal_uncached_threads_closed
        PASS [   0.085s] codex-tui app::session_lifecycle::tests::closed_state_for_thread_read_error_preserves_live_state_without_cache_on_transient_error
        PASS [   0.085s] codex-tui app::plugin_mentions::tests::plugin_mentions_use_plugin_list_summaries_and_gui_eligibility
        PASS [   0.076s] codex-tui app::session_lifecycle::tests::include_turns_fallback_detection_handles_unmaterialized_and_ephemeral_threads
        PASS [   0.067s] codex-tui app::session_lifecycle::tests::terminal_thread_read_error_detection_ignores_transient_failures
        PASS [   0.072s] codex-tui app::session_lifecycle::tests::terminal_thread_read_error_detection_matches_not_loaded_errors
        PASS [   0.069s] codex-tui app::side::tests::side_boundary_prompt_marks_inherited_history_reference_only
        PASS [   0.075s] codex-tui app::side::tests::side_developer_instructions_appends_existing_policy
        PASS [   0.068s] codex-tui app::side::tests::side_start_error_message_uses_generic_start_wording
        PASS [   0.075s] codex-tui app::side::tests::side_start_error_message_explains_missing_first_prompt
        PASS [   0.078s] codex-tui app::startup_prompts::tests::normalize_harness_overrides_resolves_relative_add_dirs
        PASS [   0.064s] codex-tui app::startup_prompts::tests::skill_load_warning_state_displays_new_message_for_active_path
        PASS [   0.076s] codex-tui app::startup_prompts::tests::skill_load_warning_state_clear_allows_active_error_again
        PASS [   0.080s] codex-tui app::startup_prompts::tests::skill_load_warning_state_reemits_after_error_clears
        PASS [   0.083s] codex-tui app::startup_prompts::tests::skill_load_warning_state_suppresses_repeated_active_errors
        PASS [   0.131s] codex-tui app::tests::active_non_primary_shutdown_target_returns_ids_for_non_primary_shutdown
        PASS [   0.125s] codex-tui app::tests::active_non_primary_shutdown_target_returns_none_for_non_shutdown_event
        PASS [   0.090s] codex-tui app::tests::active_turn_not_steerable_turn_error_extracts_structured_server_error
        PASS [   0.090s] codex-tui app::tests::active_turn_steer_race_detects_missing_active_turn
        PASS [   0.144s] codex-tui app::tests::active_non_primary_shutdown_target_returns_none_for_primary_thread_shutdown
        PASS [   0.071s] codex-tui app::tests::active_turn_steer_race_extracts_actual_turn_id_from_mismatch
        PASS [   0.136s] codex-tui app::tests::active_non_primary_shutdown_target_still_switches_for_other_pending_exit_thread
        PASS [   0.162s] codex-tui app::tests::active_non_primary_shutdown_target_returns_none_when_shutdown_exit_is_pending
        PASS [   0.145s] codex-tui app::tests::active_turn_id_for_thread_uses_snapshot_turns
        PASS [   0.145s] codex-tui app::tests::apply_permission_profile_selection_preserves_loader_overrides
        PASS [   0.135s] codex-tui app::tests::backtrack_remote_image_only_selection_clears_existing_composer_draft
        PASS [   0.183s] codex-tui app::tests::backtrack_esc_does_not_steal_empty_vim_insert_escape
        PASS [   0.145s] codex-tui app::tests::backtrack_resubmit_preserves_data_image_urls_in_user_turn
        PASS [   0.166s] codex-tui app::tests::backtrack_selection_with_duplicate_history_targets_unique_turn
        PASS [   0.141s] codex-tui app::tests::capped_resize_reflow_renders_recent_suffix_only
        PASS [   0.121s] codex-tui app::tests::clear_only_ui_reset_allows_active_skill_warning_to_render_again
        PASS [   0.102s] codex-tui app::tests::collab_receiver_notification_caches_thread_without_app_server_read
        PASS [   0.127s] codex-tui app::tests::clear_only_ui_reset_preserves_chat_session_state
        PASS [   0.778s] codex-tui app::history_ui::tests::desktop_thread_open_error_history_snapshot
        PASS [   0.117s] codex-tui app::tests::collab_receiver_notification_does_not_cache_not_found_thread
        PASS [   0.100s] codex-tui app::tests::discard_closed_side_thread_removes_local_state_without_server_rpc
        PASS [   0.111s] codex-tui app::tests::enqueue_primary_thread_session_replays_turns_before_initial_prompt_submit
  TRY 1 FAIL [   0.957s] codex-tui app::history_ui::tests::desktop_thread_opened_history_snapshot
  stdout ---

    running 1 test
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Snapshot Summary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Snapshot file: tui\src\app\snapshots\codex_tui__app__history_ui__tests__desktop_thread_opened_history.snap
    Snapshot: desktop_thread_opened_history
    Source: C:\CodexLab\codex\codex-rs:9
    ───────────────────────────────────────────────────────────────────────────────
    Expression: render_cell(&cell)
    ───────────────────────────────────────────────────────────────────────────────
    -old snapshot
    +new results
    ────────────┬──────────────────────────────────────────────────────────────────
        1       │-• Opened this session in Codex Desktop.
              1 │+ⓘ Opened this session in Codex Desktop.
    ────────────┴──────────────────────────────────────────────────────────────────
    To update snapshots run `cargo insta review`
    Stopped on the first failure. Run `cargo insta test` to run all snapshots.
    test app::history_ui::tests::desktop_thread_opened_history_snapshot ... FAILED

    failures:

    failures:
        app::history_ui::tests::desktop_thread_opened_history_snapshot

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.89s

  stderr ---
    stored new snapshot C:\CodexLab\codex\codex-rs\tui\src\app\snapshots\codex_tui__app__history_ui__tests__desktop_thread_opened_history.snap.new

    thread 'app::history_ui::tests::desktop_thread_opened_history_snapshot' (16824) panicked at C:\Users\JohnDell\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\insta-1.46.3\src\runtime.rs:719:13:
    snapshot assertion for 'desktop_thread_opened_history' failed in line 9
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.131s] codex-tui app::tests::enqueue_thread_event_does_not_block_when_channel_full
        PASS [   0.284s] codex-tui app::tests::enqueue_primary_thread_session_replays_buffered_approval_after_attach
        PASS [   0.719s] codex-tui app::tests::agent_picker_item_name_snapshot
        PASS [   0.185s] codex-tui app::tests::feedback_submission_for_inactive_thread_replays_into_origin_thread
        PASS [   0.145s] codex-tui app::tests::feedback_submission_without_thread_emits_error_history_cell
  TRY 1 FAIL [   0.938s] codex-tui app::startup_prompts::tests::repeated_active_skill_load_warning_renders_once
  stdout ---

    running 1 test
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Snapshot Summary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Snapshot: repeated_active_skill_load_warning_renders_once
    Source: C:\CodexLab\codex\codex-rs:564
    ───────────────────────────────────────────────────────────────────────────────
    Expression: rendered
    ───────────────────────────────────────────────────────────────────────────────
    -old snapshot
    +new results
    ────────────┬──────────────────────────────────────────────────────────────────
        1       │-âš  Skipped loading 1 skill(s) due to invalid SKILL.md files.
        2       │-âš  /repo/.codex/skills/abc/SKILL.md: invalid description
              1 │+⚠ Skipped loading 1 skill(s) due to invalid SKILL.md files.
              2 │+⚠ /repo/.codex/skills/abc/SKILL.md: invalid description
    ────────────┴──────────────────────────────────────────────────────────────────
    To update snapshots run `cargo insta review`
    Stopped on the first failure. Run `cargo insta test` to run all snapshots.
    test app::startup_prompts::tests::repeated_active_skill_load_warning_renders_once ... FAILED

    failures:

    failures:
        app::startup_prompts::tests::repeated_active_skill_load_warning_renders_once

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.87s

  stderr ---

    thread 'app::startup_prompts::tests::repeated_active_skill_load_warning_renders_once' (4048) panicked at C:\Users\JohnDell\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\insta-1.46.3\src\runtime.rs:719:13:
    snapshot assertion for 'repeated_active_skill_load_warning_renders_once' failed in line 564
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.766s] codex-tui app::tests::bypass_hook_trust_startup_warning_snapshot
        PASS [   0.131s] codex-tui app::tests::first_cancelled_turn_edit_restores_prompt_without_local_history
        PASS [   0.108s] codex-tui app::tests::fresh_session_config_uses_current_service_tier
        PASS [   0.131s] codex-tui app::tests::handle_mcp_inventory_result_respects_origin_thread
        PASS [   0.134s] codex-tui app::tests::height_shrink_schedules_resize_reflow
        PASS [   0.093s] codex-tui app::tests::history_lookup_response_is_routed_to_requesting_thread
        PASS [   0.769s] codex-tui app::tests::cancelled_turn_edit_restores_prompt_and_rolls_back_latest_turn
        LEAK [   0.938s] codex-tui app::tests::attach_live_thread_for_selection_rejects_unmaterialized_fallback_threads
        LEAK [   0.948s] codex-tui app::tests::attach_live_thread_for_selection_rejects_empty_non_ephemeral_fallback_threads
        PASS [   0.132s] codex-tui app::tests::inactive_thread_exec_approval_preserves_context
        PASS [   0.159s] codex-tui app::tests::inactive_thread_exec_approval_splits_shell_wrapped_command
        PASS [   0.155s] codex-tui app::tests::inactive_thread_invalid_url_elicitation_is_declined
        PASS [   0.159s] codex-tui app::tests::inactive_thread_settings_notification_updates_cached_collaboration_mode
        PASS [   0.167s] codex-tui app::tests::inactive_thread_permissions_approval_preserves_file_system_permissions
        PASS [   0.297s] codex-tui app::tests::inactive_thread_approval_badge_clears_after_turn_completion_notification
        PASS [   0.317s] codex-tui app::tests::inactive_thread_approval_bubbles_into_active_view
        PASS [   0.158s] codex-tui app::tests::inactive_thread_started_notification_initializes_replay_session
        PASS [   0.132s] codex-tui app::tests::inactive_thread_started_notification_preserves_primary_model_when_path_missing
        LEAK [   0.852s] codex-tui app::tests::discard_side_thread_keeps_local_state_when_server_close_fails
        PASS [   0.108s] codex-tui app::tests::inactive_thread_url_elicitation_routes_to_app_link
        PASS [   0.066s] codex-tui app::tests::model_catalog::model_migration_prompt_only_shows_for_deprecated_models
        PASS [   0.095s] codex-tui app::tests::model_catalog::accepted_model_migration_persists_target_default_reasoning_effort
        PASS [   0.061s] codex-tui app::tests::model_catalog::model_migration_prompt_respects_hide_flag_and_self_target
        PASS [   0.127s] codex-tui app::tests::initial_replay_buffer_keeps_recent_rows_when_row_cap_present
        PASS [   0.062s] codex-tui app::tests::model_catalog::model_migration_prompt_skips_when_target_missing_or_hidden
        LEAK [   0.951s] codex-tui app::tests::discard_side_thread_removes_agent_navigation_entry
        PASS [   0.063s] codex-tui app::tests::model_catalog::select_model_availability_nux_returns_none_when_all_models_are_exhausted
        PASS [   0.067s] codex-tui app::tests::model_catalog::select_model_availability_nux_skips_missing_and_exhausted_models
        PASS [   0.360s] codex-tui app::tests::inactive_thread_file_change_approval_recovers_buffered_changes
        PASS [   0.082s] codex-tui app::tests::model_catalog::select_model_availability_nux_picks_only_eligible_model
        PASS [   0.085s] codex-tui app::tests::model_catalog::select_model_availability_nux_uses_existing_model_order_as_priority
        PASS [   0.124s] codex-tui app::tests::model_catalog::prepare_startup_tooltip_override_persists_model_availability_nux_count
  TRY 2 FAIL [   0.961s] codex-tui app::history_ui::tests::desktop_thread_opened_history_snapshot
  stdout ---

    running 1 test
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Snapshot Summary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Snapshot file: tui\src\app\snapshots\codex_tui__app__history_ui__tests__desktop_thread_opened_history.snap
    Snapshot: desktop_thread_opened_history
    Source: C:\CodexLab\codex\codex-rs:9
    ───────────────────────────────────────────────────────────────────────────────
    Expression: render_cell(&cell)
    ───────────────────────────────────────────────────────────────────────────────
    -old snapshot
    +new results
    ────────────┬──────────────────────────────────────────────────────────────────
        1       │-• Opened this session in Codex Desktop.
              1 │+ⓘ Opened this session in Codex Desktop.
    ────────────┴──────────────────────────────────────────────────────────────────
    To update snapshots run `cargo insta review`
    Stopped on the first failure. Run `cargo insta test` to run all snapshots.
    test app::history_ui::tests::desktop_thread_opened_history_snapshot ... FAILED

    failures:

    failures:
        app::history_ui::tests::desktop_thread_opened_history_snapshot

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.88s

  stderr ---
    stored new snapshot C:\CodexLab\codex\codex-rs\tui\src\app\snapshots\codex_tui__app__history_ui__tests__desktop_thread_opened_history.snap.new

    thread 'app::history_ui::tests::desktop_thread_opened_history_snapshot' (14588) panicked at C:\Users\JohnDell\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\insta-1.46.3\src\runtime.rs:719:13:
    snapshot assertion for 'desktop_thread_opened_history' failed in line 9
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

  TRY 2 FAIL [   0.883s] codex-tui app::startup_prompts::tests::repeated_active_skill_load_warning_renders_once
  stdout ---

    running 1 test
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Snapshot Summary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Snapshot: repeated_active_skill_load_warning_renders_once
    Source: C:\CodexLab\codex\codex-rs:564
    ───────────────────────────────────────────────────────────────────────────────
    Expression: rendered
    ───────────────────────────────────────────────────────────────────────────────
    -old snapshot
    +new results
    ────────────┬──────────────────────────────────────────────────────────────────
        1       │-âš  Skipped loading 1 skill(s) due to invalid SKILL.md files.
        2       │-âš  /repo/.codex/skills/abc/SKILL.md: invalid description
              1 │+⚠ Skipped loading 1 skill(s) due to invalid SKILL.md files.
              2 │+⚠ /repo/.codex/skills/abc/SKILL.md: invalid description
    ────────────┴──────────────────────────────────────────────────────────────────
    To update snapshots run `cargo insta review`
    Stopped on the first failure. Run `cargo insta test` to run all snapshots.
    test app::startup_prompts::tests::repeated_active_skill_load_warning_renders_once ... FAILED

    failures:

    failures:
        app::startup_prompts::tests::repeated_active_skill_load_warning_renders_once

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.82s

  stderr ---

    thread 'app::startup_prompts::tests::repeated_active_skill_load_warning_renders_once' (23672) panicked at C:\Users\JohnDell\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\insta-1.46.3\src\runtime.rs:719:13:
    snapshot assertion for 'repeated_active_skill_load_warning_renders_once' failed in line 564
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.098s] codex-tui app::tests::queued_rollback_syncs_overlay_and_clears_deferred_history
        PASS [   0.799s] codex-tui app::tests::model_catalog::model_migration_prompt_shows_for_hidden_model
        LEAK [   0.884s] codex-tui app::tests::interrupt_without_active_turn_is_treated_as_handled
        LEAK [   0.819s] codex-tui app::tests::new_session_requests_shutdown_for_previous_conversation
        LEAK [   0.925s] codex-tui app::tests::open_agent_picker_marks_loaded_threads_open
        LEAK [   0.939s] codex-tui app::tests::open_agent_picker_keeps_missing_threads_for_replay
        LEAK [   0.959s] codex-tui app::tests::open_agent_picker_prompts_to_enable_multi_agent_when_disabled
        LEAK [   0.986s] codex-tui app::tests::open_agent_picker_allows_existing_agent_threads_when_feature_is_disabled
        LEAK [   0.951s] codex-tui app::tests::open_agent_picker_prunes_terminal_metadata_only_threads
        LEAK [   0.984s] codex-tui app::tests::open_agent_picker_preserves_cached_metadata_for_replay_threads
        LEAK [   1.084s] codex-tui app::tests::open_agent_picker_marks_terminal_read_errors_closed
        PASS [   0.120s] codex-tui app::tests::refreshed_snapshot_session_persists_resumed_turns
        PASS [   0.127s] codex-tui app::tests::refresh_pending_thread_approvals_only_lists_inactive_threads
        PASS [   0.152s] codex-tui app::tests::replace_chat_widget_reseeds_collab_agent_metadata_for_replay
        PASS [   0.185s] codex-tui app::tests::replay_only_thread_keeps_restored_queue_visible
        PASS [   0.234s] codex-tui app::tests::replay_thread_snapshot_in_progress_turn_restores_running_state_without_input_state
        PASS [   0.184s] codex-tui app::tests::replay_thread_snapshot_replays_turn_history_in_order
        PASS [   0.259s] codex-tui app::tests::replay_thread_snapshot_keeps_queue_when_running_state_only_comes_from_snapshot
        LEAK [   1.049s] codex-tui app::tests::override_turn_context_sends_thread_settings_update
        PASS [   0.266s] codex-tui app::tests::replay_thread_snapshot_in_progress_turn_restores_running_queue_state
        PASS [   0.143s] codex-tui app::tests::replay_thread_snapshot_restores_collaboration_mode_without_input
        PASS [   0.274s] codex-tui app::tests::replay_thread_snapshot_does_not_submit_queue_before_replay_catches_up
        PASS [   0.152s] codex-tui app::tests::replay_thread_snapshot_restores_collaboration_mode_for_draft_submit
        PASS [   0.163s] codex-tui app::tests::replay_thread_snapshot_restores_draft_and_queued_input
        PASS [   0.330s] codex-tui app::tests::replay_snapshot_with_pending_request_suppresses_replay_notices
        PASS [   0.156s] codex-tui app::tests::replay_thread_snapshot_restores_pending_pastes_for_submit
        PASS [   0.095s] codex-tui app::tests::session_summary::session_summary_includes_resume_hint_for_persisted_rollout
        LEAK [   0.968s] codex-tui app::tests::refresh_agent_picker_thread_liveness_prunes_closed_metadata_only_threads
        PASS [   0.107s] codex-tui app::tests::session_start_error_surfaces_archived_guidance_without_rollout_path
        PASS [   0.150s] codex-tui app::tests::replayed_interrupted_turn_restores_queued_input_to_composer
        PASS [   0.132s] codex-tui app::tests::resize_reflow_wraps_transcript_early_when_pet_is_enabled
        PASS [   0.068s] codex-tui app::tests::session_summary::session_summary_names_picker_item_when_thread_has_name
        PASS [   0.140s] codex-tui app::tests::reset_thread_event_state_aborts_listener_tasks
        PASS [   0.145s] codex-tui app::tests::replayed_turn_complete_submits_restored_queued_follow_up
        PASS [   0.143s] codex-tui app::tests::resolved_buffered_approval_does_not_become_actionable_after_drain
        PASS [   0.045s] codex-tui app::tests::session_summary::session_summary_skips_when_no_usage_or_resume_hint
        PASS [   0.058s] codex-tui app::tests::session_summary::session_summary_skips_resume_hint_until_rollout_exists
        PASS [   0.108s] codex-tui app::tests::should_attach_live_thread_for_selection_skips_closed_metadata_only_threads
        PASS [   0.172s] codex-tui app::tests::side_conversations_reject_backtrack_esc_without_stealing_vim_insert_escape
        PASS [   0.207s] codex-tui app::tests::side_discard_selection_keeps_current_side_thread
        PASS [   0.179s] codex-tui app::tests::side_fork_config_is_ephemeral_and_appends_developer_guardrails
        PASS [   0.208s] codex-tui app::tests::side_fork_config_inherits_parent_thread_runtime_settings
        PASS [   0.202s] codex-tui app::tests::side_parent_status_prioritizes_input_over_approval
        PASS [   0.340s] codex-tui app::tests::side_defers_parent_approval_overlay_until_parent_replay
        PASS [   0.348s] codex-tui app::tests::side_defers_subagent_approval_overlay_until_side_exits
        PASS [   0.125s] codex-tui app::tests::side_thread_snapshot_hides_forked_parent_transcript
        PASS [   0.133s] codex-tui app::tests::side_parent_status_tracks_parent_turn_lifecycle
        PASS [   0.150s] codex-tui app::tests::side_restore_user_message_puts_inline_question_back_in_composer
        PASS [   0.169s] codex-tui app::tests::side_start_block_message_tracks_open_side_conversation
        PASS [   0.196s] codex-tui app::tests::side_thread_snapshot_does_not_refresh_from_fork_history
        PASS [   0.131s] codex-tui app::tests::side_thread_snapshot_skips_session_header_preamble
        PASS [   0.130s] codex-tui app::tests::startup::ignore_same_thread_resume_allows_reattaching_displayed_inactive_thread
        PASS [   0.069s] codex-tui app::tests::startup::startup_paused_goal_prompt_gate_is_only_for_quiet_resume
        PASS [   0.147s] codex-tui app::tests::startup::ignore_same_thread_resume_reports_noop_for_current_thread
        PASS [   0.083s] codex-tui app::tests::startup::startup_waiting_gate_holds_active_thread_events_until_primary_thread_configured
        PASS [   0.102s] codex-tui app::tests::startup::startup_waiting_gate_is_only_for_fresh_or_exit_session_selection
        PASS [   0.115s] codex-tui app::tests::startup::startup_waiting_gate_not_applied_for_resume_or_fork_session_selection
        PASS [   0.109s] codex-tui app::tests::thread_rollback_response_discards_queued_active_thread_events
        PASS [   0.127s] codex-tui app::tests::thread_read_session_state_does_not_reuse_primary_permission_profile
        PASS [   0.135s] codex-tui app::tests::thread_setting_update_params_sync_model_and_default_reasoning
        PASS [   0.130s] codex-tui app::tests::thread_switch_replay_buffer_is_disabled_without_row_cap
        LEAK [   0.906s] codex-tui app::tests::reset_memories_clears_local_memory_directories
        PASS [   0.130s] codex-tui app::tests::token_usage_update_refreshes_status_line_with_runtime_context_window
        PASS [   0.146s] codex-tui app::tests::thread_switch_replay_buffer_uses_transcript_tail_mode_when_row_cap_present
        PASS [   0.128s] codex-tui app::tests::uncapped_resize_reflow_renders_all_cells_under_row_limit
        PASS [   0.129s] codex-tui app::tests::uncapped_resize_reflow_renders_all_cells_when_row_cap_absent
        PASS [   0.920s] codex-tui app::tests::side_backtrack_rejection_reports_unavailable_message_snapshot
        LEAK [   0.976s] codex-tui app::tests::shutdown_first_exit_returns_immediate_exit_when_shutdown_submit_fails
        PASS [   0.037s] codex-tui app::thread_events::tests::thread_event_store_clear_active_turn_id_resets_cached_turn
        PASS [   0.039s] codex-tui app::thread_events::tests::thread_event_store_rebase_preserves_hook_notifications
        PASS [   0.040s] codex-tui app::thread_events::tests::thread_event_store_rebase_preserves_resolved_request_state
        PASS [   0.041s] codex-tui app::thread_events::tests::thread_event_store_restores_active_turn_from_snapshot_turns
        PASS [   0.034s] codex-tui app::thread_events::tests::thread_event_store_tracks_active_turn_lifecycle
        PASS [   0.038s] codex-tui app::thread_goal_actions::tests::completed_goal_does_not_require_replace_confirmation
        LEAK [   1.240s] codex-tui app::tests::shutdown_first_exit_uses_app_server_shutdown_without_submitting_op
        PASS [   0.050s] codex-tui app::thread_goal_actions::tests::thread_goal_error_message_explains_temporary_session
        PASS [   0.048s] codex-tui app::thread_goal_actions::tests::thread_goal_error_message_preserves_generic_failure_context
        PASS [   0.040s] codex-tui app::thread_goal_actions::tests::unfinished_goals_require_replace_confirmation
        PASS [   0.058s] codex-tui app::thread_routing::tests::turn_permissions_preserve_server_snapshot_without_local_override
        PASS [   0.040s] codex-tui app::thread_routing::tests::turn_permissions_send_legacy_sandbox_for_local_override
        PASS [   1.262s] codex-tui app::tests::side_thread_ignores_global_mcp_startup_notifications
        LEAK [   1.153s] codex-tui app::tests::startup::stale_startup_thread_started_removes_local_routing_state
        PASS [   0.070s] codex-tui app::thread_routing::tests::turn_permissions_use_active_profile_when_available
        PASS [   0.068s] codex-tui app::thread_session_state::tests::permission_settings_sync_preserves_active_profile_only_rules
        PASS [   0.065s] codex-tui app::thread_session_state::tests::permission_settings_sync_updates_active_snapshot_without_rewriting_side_thread
        PASS [   0.069s] codex-tui app::thread_session_state::tests::service_tier_sync_updates_active_cached_session
        PASS [   0.031s] codex-tui app_backtrack::tests::agent_group_count_ignores_context_compacted_marker
        PASS [   0.032s] codex-tui app_backtrack::tests::backtrack_target_requires_user_message
        PASS [   0.087s] codex-tui app::thread_session_state::tests::thread_read_fallback_uses_active_permission_settings
        PASS [   0.030s] codex-tui app_backtrack::tests::trim_drop_last_n_user_turns_allows_overflow
        PASS [   0.040s] codex-tui app_backtrack::tests::trim_drop_last_n_user_turns_applies_rollback_semantics
        PASS [   0.516s] codex-tui app::thread_goal_actions::tests::thread_goal_ephemeral_error_message_renders_snapshot
        PASS [   0.040s] codex-tui app_backtrack::tests::trim_transcript_for_first_user_drops_user_and_newer_cells
        PASS [   0.040s] codex-tui app_backtrack::tests::trim_transcript_for_later_user_keeps_prior_history
        PASS [   0.043s] codex-tui app_backtrack::tests::trim_transcript_preserves_cells_before_selected_user
        PASS [   0.043s] codex-tui app_server_approval_conversions::tests::converts_file_update_changes_to_display
        PASS [   0.043s] codex-tui app_server_approval_conversions::tests::converts_request_permissions_into_canonical_granted_permissions
        PASS [   0.044s] codex-tui app_server_approval_conversions::tests::converts_request_permissions_into_granted_permissions
        PASS [   0.047s] codex-tui app_server_session::tests::app_server_rate_limit_snapshots_deduplicates_top_level_limit_from_map
        PASS [   0.047s] codex-tui app_server_session::tests::embedded_turn_permissions_select_profile_id_only
        PASS [   0.067s] codex-tui app_server_session::tests::config_request_overrides_preserve_implicit_personality_default
        PASS [   0.065s] codex-tui app_server_session::tests::embedded_thread_response_uses_local_config_profile
        PASS [   0.041s] codex-tui app_server_session::tests::embedded_turn_permissions_use_active_profile_selection
        PASS [   0.046s] codex-tui app_server_session::tests::legacy_turn_permissions_project_to_sandbox_when_explicitly_overridden
        PASS [   0.056s] codex-tui app_server_session::tests::remote_thread_response_uses_legacy_sandbox_fallback
        PASS [   0.035s] codex-tui app_server_session::tests::remote_turn_permissions_preserve_active_profile_selection
        PASS [   0.035s] codex-tui app_server_session::tests::sandbox_mode_does_not_project_non_cwd_write_roots_for_remote_sessions
        PASS [   0.052s] codex-tui app_server_session::tests::resume_response_restores_turns_from_thread_items
        PASS [   0.036s] codex-tui app_server_session::tests::sandbox_mode_projects_cwd_write_for_remote_sessions
        LEAK [   1.561s] codex-tui app::tests::startup::startup_thread_start_failure_returns_error
        PASS [   0.043s] codex-tui app_server_session::tests::status_account_display_from_auth_mode_uses_remapped_plan_labels
        PASS [   0.063s] codex-tui app_server_session::tests::session_configured_preserves_fork_source_thread_id
        LEAK [   1.615s] codex-tui app::tests::startup::startup_thread_started_submits_queued_startup_input
        PASS [   0.114s] codex-tui app_server_session::tests::session_configured_populates_history_metadata
        PASS [   0.105s] codex-tui app_server_session::tests::terminal_visualization_instructions_are_gated_for_all_tui_thread_flows
        PASS [   0.109s] codex-tui app_server_session::tests::thread_lifecycle_params_forward_config_overrides_and_service_tier
        PASS [   0.114s] codex-tui app_server_session::tests::thread_fork_params_forward_instruction_overrides
        PASS [   0.066s] codex-tui app_server_session::tests::thread_settings_update_compat_detects_unsupported_errors
        PASS [   0.117s] codex-tui app_server_session::tests::thread_lifecycle_params_forward_explicit_remote_cwd_override_for_remote_sessions
        PASS [   0.164s] codex-tui app_server_session::tests::thread_lifecycle_params_omit_cwd_without_remote_override_for_remote_sessions
        PASS [   0.115s] codex-tui app_server_session::tests::turn_permissions_preserve_thread_permissions_without_override
        PASS [   0.139s] codex-tui app_server_session::tests::thread_start_params_can_mark_clear_source
        PASS [   0.097s] codex-tui ascii_animation::tests::frame_tick_must_be_nonzero
        PASS [   0.177s] codex-tui app_server_session::tests::thread_start_params_include_cwd_for_embedded_sessions
        PASS [   0.104s] codex-tui bottom_pane::app_link_view::tests::codex_apps_auth_url_elicitation_builds_auth_app_link_params
        PASS [   0.103s] codex-tui bottom_pane::app_link_view::tests::codex_apps_auth_url_elicitation_rejects_untrusted_urls
        PASS [   0.162s] codex-tui auto_review_denials::tests::keeps_only_ten_most_recent_denials
        PASS [   0.105s] codex-tui bottom_pane::app_link_view::tests::declined_tool_suggestion_resolves_elicitation_decline
        LEAK [   1.594s] codex-tui app::tests::update_memory_settings_persists_and_updates_widget_config
        LEAK [   1.730s] codex-tui app::tests::update_feature_flags_disabling_guardian_clears_manual_review_policy_without_history
  TRY 1 FAIL [   0.867s] codex-tui app_backtrack::tests::backtrack_unavailable_info_message_snapshot
  stdout ---

    running 1 test
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Snapshot Summary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Snapshot file: tui\src\snapshots\codex_tui__app_backtrack__tests__backtrack_unavailable_info_message_snapshot.snap
    Snapshot: backtrack_unavailable_info_message_snapshot
    Source: C:\CodexLab\codex\codex-rs:982
    ───────────────────────────────────────────────────────────────────────────────
    Expression: rendered
    ───────────────────────────────────────────────────────────────────────────────
    -old snapshot
    +new results
    ────────────┬──────────────────────────────────────────────────────────────────
        1       │-• No previous message to edit.
              1 │+ⓘ No previous message to edit.
    ────────────┴──────────────────────────────────────────────────────────────────
    To update snapshots run `cargo insta review`
    Stopped on the first failure. Run `cargo insta test` to run all snapshots.
    test app_backtrack::tests::backtrack_unavailable_info_message_snapshot ... FAILED

    failures:

    failures:
        app_backtrack::tests::backtrack_unavailable_info_message_snapshot

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.83s

  stderr ---
    stored new snapshot C:\CodexLab\codex\codex-rs\tui\src\snapshots\codex_tui__app_backtrack__tests__backtrack_unavailable_info_message_snapshot.snap.new

    thread 'app_backtrack::tests::backtrack_unavailable_info_message_snapshot' (18084) panicked at C:\Users\JohnDell\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\insta-1.46.3\src\runtime.rs:719:13:
    snapshot assertion for 'backtrack_unavailable_info_message_snapshot' failed in line 982
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.069s] codex-tui bottom_pane::app_link_view::tests::generic_url_elicitation_rejects_untrusted_urls
        PASS [   0.075s] codex-tui bottom_pane::app_link_view::tests::generic_url_elicitation_resolves_without_connector_refresh
        PASS [   0.087s] codex-tui bottom_pane::app_link_view::tests::enable_tool_suggestion_resolves_elicitation_after_enable
        LEAK [   1.739s] codex-tui app::tests::update_feature_flags_enabling_guardian_overrides_explicit_manual_review_policy
        LEAK [   1.772s] codex-tui app::tests::update_feature_flags_disabling_guardian_clears_review_policy_and_restores_default
        PASS [   0.061s] codex-tui bottom_pane::app_link_view::tests::horizontal_list_keys_move_action_selection
        PASS [   0.054s] codex-tui bottom_pane::app_link_view::tests::install_confirmation_does_not_split_long_url_like_token_without_scheme
        PASS [   0.062s] codex-tui bottom_pane::app_link_view::tests::install_tool_suggestion_resolves_elicitation_after_confirmation
        PASS [   0.087s] codex-tui bottom_pane::app_link_view::tests::install_confirmation_render_keeps_url_tail_visible_when_narrow
        LEAK [   1.776s] codex-tui app::tests::update_feature_flags_enabling_guardian_selects_auto_review
        PASS [   0.062s] codex-tui bottom_pane::app_link_view::tests::installed_app_has_toggle_action
        PASS [   0.061s] codex-tui bottom_pane::app_link_view::tests::non_codex_apps_url_elicitation_builds_generic_app_link_params
        PASS [   0.048s] codex-tui bottom_pane::app_link_view::tests::regular_app_link_does_not_require_terminal_title_action
        PASS [   0.057s] codex-tui bottom_pane::app_link_view::tests::remapped_horizontal_list_keys_control_action_selection
        PASS [   0.056s] codex-tui bottom_pane::app_link_view::tests::resolved_tool_suggestion_ignores_non_matching_request
        PASS [   0.072s] codex-tui bottom_pane::app_link_view::tests::resolved_tool_suggestion_dismisses_matching_view
        PASS [   0.065s] codex-tui bottom_pane::app_link_view::tests::toggle_action_sends_set_app_enabled_and_updates_label
        PASS [   0.043s] codex-tui bottom_pane::app_link_view::tests::tool_suggestion_requires_terminal_title_action
        PASS [   0.054s] codex-tui bottom_pane::approval_overlay::tests::additional_permissions_exec_options_hide_execpolicy_amendment
        PASS [   0.073s] codex-tui bottom_pane::approval_overlay::tests::additional_permissions_rule_shows_non_path_file_system_entries
        PASS [   0.079s] codex-tui bottom_pane::approval_overlay::tests::additional_permissions_rule_uses_workspace_roots_label
        PASS [   0.086s] codex-tui bottom_pane::approval_overlay::tests::apply_patch_prompt_with_thread_label_omits_command_line
        PASS [   0.050s] codex-tui bottom_pane::approval_overlay::tests::configured_list_cancel_cancels_mcp_elicitation
        PASS [   0.268s] codex-tui bottom_pane::approval_overlay::tests::additional_permissions_prompt_shows_permission_rule_line
        PASS [   0.290s] codex-tui bottom_pane::approval_overlay::tests::configured_list_cancel_aborts_exec_approval
        PASS [   0.259s] codex-tui bottom_pane::approval_overlay::tests::configured_open_thread_shortcut_opens_source_thread
        PASS [   0.822s] codex-tui bottom_pane::app_link_view::tests::auth_suggestion_with_reason_snapshot
        PASS [   0.769s] codex-tui bottom_pane::app_link_view::tests::enable_suggestion_with_reason_snapshot
        PASS [   0.741s] codex-tui bottom_pane::app_link_view::tests::generic_url_elicitation_confirmation_snapshot
        PASS [   0.248s] codex-tui bottom_pane::approval_overlay::tests::ctrl_c_aborts_and_clears_queue
        PASS [   0.088s] codex-tui bottom_pane::approval_overlay::tests::esc_cancels_mcp_elicitation
        PASS [   0.785s] codex-tui bottom_pane::app_link_view::tests::generic_url_elicitation_snapshot
        PASS [   0.738s] codex-tui bottom_pane::app_link_view::tests::install_suggestion_with_reason_snapshot
  TRY 2 FAIL [   0.827s] codex-tui app_backtrack::tests::backtrack_unavailable_info_message_snapshot
  stdout ---

    running 1 test
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Snapshot Summary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Snapshot file: tui\src\snapshots\codex_tui__app_backtrack__tests__backtrack_unavailable_info_message_snapshot.snap
    Snapshot: backtrack_unavailable_info_message_snapshot
    Source: C:\CodexLab\codex\codex-rs:982
    ───────────────────────────────────────────────────────────────────────────────
    Expression: rendered
    ───────────────────────────────────────────────────────────────────────────────
    -old snapshot
    +new results
    ────────────┬──────────────────────────────────────────────────────────────────
        1       │-• No previous message to edit.
              1 │+ⓘ No previous message to edit.
    ────────────┴──────────────────────────────────────────────────────────────────
    To update snapshots run `cargo insta review`
    Stopped on the first failure. Run `cargo insta test` to run all snapshots.
    test app_backtrack::tests::backtrack_unavailable_info_message_snapshot ... FAILED

    failures:

    failures:
        app_backtrack::tests::backtrack_unavailable_info_message_snapshot

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.78s

  stderr ---
    stored new snapshot C:\CodexLab\codex\codex-rs\tui\src\snapshots\codex_tui__app_backtrack__tests__backtrack_unavailable_info_message_snapshot.snap.new

    thread 'app_backtrack::tests::backtrack_unavailable_info_message_snapshot' (10068) panicked at C:\Users\JohnDell\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\insta-1.46.3\src\runtime.rs:719:13:
    snapshot assertion for 'backtrack_unavailable_info_message_snapshot' failed in line 982
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.051s] codex-tui bottom_pane::approval_overlay::tests::esc_still_cancels_elicitation_with_custom_overlap
        PASS [   0.260s] codex-tui bottom_pane::approval_overlay::tests::ctrl_shift_a_opens_fullscreen
        PASS [   0.070s] codex-tui bottom_pane::approval_overlay::tests::exec_history_cell_wraps_with_two_space_indent
        PASS [   0.082s] codex-tui bottom_pane::approval_overlay::tests::exec_history_cell_does_not_render_blank_action_for_empty_command
        PASS [   0.084s] codex-tui bottom_pane::approval_overlay::tests::generic_exec_options_can_offer_allow_for_session
        PASS [   0.283s] codex-tui bottom_pane::approval_overlay::tests::enter_sets_last_selected_index_without_dismissing
        PASS [   0.304s] codex-tui bottom_pane::approval_overlay::tests::deny_shortcut_submits_denied_exec_decision
        PASS [   0.071s] codex-tui bottom_pane::approval_overlay::tests::network_exec_options_use_expected_labels_and_hide_execpolicy_amendment
        PASS [   0.084s] codex-tui bottom_pane::approval_overlay::tests::permissions_deny_shortcut_uses_deny_keymap
        PASS [   0.069s] codex-tui bottom_pane::approval_overlay::tests::permissions_options_use_expected_labels
        PASS [   0.297s] codex-tui bottom_pane::approval_overlay::tests::exec_prefix_option_emits_execpolicy_amendment
        PASS [   0.259s] codex-tui bottom_pane::approval_overlay::tests::network_deny_shortcut_submits_policy_deny_decision
        PASS [   0.285s] codex-tui bottom_pane::approval_overlay::tests::header_includes_command_snippet
        PASS [   0.072s] codex-tui bottom_pane::approval_overlay::tests::permissions_session_shortcut_submits_session_scope
        PASS [   0.298s] codex-tui bottom_pane::approval_overlay::tests::network_access_command_history_uses_target_without_structured_context
        PASS [   0.054s] codex-tui bottom_pane::approval_overlay::tests::permissions_strict_auto_review_shortcut_submits_turn_scope_with_strict_review
        PASS [   0.299s] codex-tui bottom_pane::approval_overlay::tests::network_deny_forever_shortcut_is_not_bound
        PASS [   0.064s] codex-tui bottom_pane::chat_composer::history_search::tests::history_search_accepts_matching_entry
        PASS [   0.068s] codex-tui bottom_pane::chat_composer::history_search::tests::history_search_ctrl_c_restores_original_draft
        PASS [   0.991s] codex-tui bottom_pane::approval_overlay::tests::additional_permissions_prompt_snapshot
        PASS [   0.312s] codex-tui bottom_pane::approval_overlay::tests::o_opens_source_thread_for_cross_thread_approval
        PASS [   0.073s] codex-tui bottom_pane::chat_composer::history_search::tests::history_search_esc_resets_normal_history_navigation
        PASS [   0.073s] codex-tui bottom_pane::chat_composer::history_search::tests::history_search_esc_restores_original_draft
        PASS [   0.047s] codex-tui bottom_pane::chat_composer::history_search::tests::history_search_flushes_buffered_paste_before_snapshot
        PASS [   0.058s] codex-tui bottom_pane::chat_composer::history_search::tests::history_search_flushes_pending_first_char_before_snapshot
        PASS [   0.058s] codex-tui bottom_pane::chat_composer::history_search::tests::history_search_footer_action_hints_are_emphasized
        PASS [   0.076s] codex-tui bottom_pane::chat_composer::history_search::tests::history_search_match_ranges_are_case_insensitive
        PASS [   0.076s] codex-tui bottom_pane::chat_composer::history_search::tests::history_search_highlights_matches_until_accepted
        PASS [   0.956s] codex-tui bottom_pane::approval_overlay::tests::cross_thread_footer_hint_mentions_o_shortcut
        PASS [   0.087s] codex-tui bottom_pane::chat_composer::history_search::tests::history_search_no_match_restores_preview_but_keeps_search_open
        PASS [   0.072s] codex-tui bottom_pane::chat_composer::history_search::tests::history_search_opens_without_previewing_latest_entry
        PASS [   0.059s] codex-tui bottom_pane::chat_composer::history_search::tests::history_search_stays_on_single_match_at_boundaries
        PASS [   0.071s] codex-tui bottom_pane::chat_composer::history_search::tests::vim_normal_history_search_preview_places_cursor_on_last_char
        PASS [   0.056s] codex-tui bottom_pane::chat_composer::slash_input::tests::slash_completion_does_not_preserve_existing_draft_tail_for_other_commands
        PASS [   0.240s] codex-tui bottom_pane::approval_overlay::tests::resolved_request_dismisses_overlay_without_emitting_abort
        PASS [   0.239s] codex-tui bottom_pane::approval_overlay::tests::shortcut_triggers_selection
        PASS [   0.045s] codex-tui bottom_pane::chat_composer::tests::apply_external_edit_absorbs_bash_prefix_without_duplication
        PASS [   0.069s] codex-tui bottom_pane::chat_composer::slash_input::tests::slash_completion_does_not_turn_command_suffix_into_args
        PASS [   0.059s] codex-tui bottom_pane::chat_composer::slash_input::tests::slash_completion_preserves_existing_draft_tail_for_inline_arg_commands
        PASS [   0.070s] codex-tui bottom_pane::chat_composer::tests::apply_external_edit_can_enter_bash_mode
        PASS [   0.071s] codex-tui bottom_pane::chat_composer::tests::apply_external_edit_can_leave_bash_mode
        PASS [   0.062s] codex-tui bottom_pane::chat_composer::tests::apply_external_edit_drops_missing_attachments
        PASS [   0.058s] codex-tui bottom_pane::chat_composer::tests::apply_external_edit_limits_duplicates_to_occurrences
        PASS [   0.053s] codex-tui bottom_pane::chat_composer::tests::apply_external_edit_rebuilds_text_and_attachments
        PASS [   0.058s] codex-tui bottom_pane::chat_composer::tests::apply_external_edit_renumbers_image_placeholders
        PASS [   0.065s] codex-tui bottom_pane::chat_composer::tests::apply_history_entry_preserves_local_placeholders_after_remote_prefix
        PASS [   0.076s] codex-tui bottom_pane::chat_composer::tests::ascii_burst_treats_enter_as_newline
        PASS [   0.081s] codex-tui bottom_pane::chat_composer::tests::ascii_prefix_survives_non_ascii_followup
        PASS [   0.076s] codex-tui bottom_pane::chat_composer::tests::attach_image_after_remote_prefix_uses_offset_label
        PASS [   0.084s] codex-tui bottom_pane::chat_composer::tests::attach_image_and_submit_includes_local_image_paths
        PASS [   0.088s] codex-tui bottom_pane::chat_composer::tests::attach_image_without_text_submits_empty_text_and_images
        PASS [   0.080s] codex-tui bottom_pane::chat_composer::tests::backspace_with_multibyte_text_before_placeholder_does_not_panic
        PASS [   0.074s] codex-tui bottom_pane::chat_composer::tests::bang_enters_shell_mode_in_vim_normal_mode
        PASS [   0.089s] codex-tui bottom_pane::chat_composer::tests::bare_slash_command_can_be_recalled_after_recording_pending_history
        PASS [   0.088s] codex-tui bottom_pane::chat_composer::tests::base_footer_mode_tracks_empty_state_after_quit_hint_expires
        PASS [   0.072s] codex-tui bottom_pane::chat_composer::tests::bound_at_mentions_do_not_block_arrow_navigation
        PASS [   0.078s] codex-tui bottom_pane::chat_composer::tests::burst_paste_fast_large_inserts_placeholder_on_flush
        PASS [   0.082s] codex-tui bottom_pane::chat_composer::tests::burst_paste_fast_small_buffers_and_flushes_on_stop
        PASS [   0.086s] codex-tui bottom_pane::chat_composer::tests::clear_for_ctrl_c_preserves_image_draft_state
        PASS [   0.080s] codex-tui bottom_pane::chat_composer::tests::clear_for_ctrl_c_preserves_pending_paste_history_entry
        PASS [   0.162s] codex-tui bottom_pane::chat_composer::tests::bang_prefixed_slash_text_submits_literal_shell_command
        PASS [   0.081s] codex-tui bottom_pane::chat_composer::tests::clear_for_ctrl_c_records_cleared_draft
        PASS [   0.121s] codex-tui bottom_pane::chat_composer::tests::clear_for_ctrl_c_preserves_remote_offset_image_labels
        PASS [   0.076s] codex-tui bottom_pane::chat_composer::tests::current_text_with_pending_expands_placeholders
        PASS [   0.073s] codex-tui bottom_pane::chat_composer::tests::delete_selected_remote_image_relabels_local_placeholders
        PASS [   0.099s] codex-tui bottom_pane::chat_composer::tests::current_text_with_pending_expands_overlapping_placeholders
        PASS [   0.079s] codex-tui bottom_pane::chat_composer::tests::deleting_duplicate_length_pastes_removes_only_target
        PASS [   0.055s] codex-tui bottom_pane::chat_composer::tests::deleting_one_of_duplicate_image_placeholders_removes_one_entry
        PASS [   0.071s] codex-tui bottom_pane::chat_composer::tests::deleting_reordered_image_one_renumbers_text_in_place
        PASS [   0.072s] codex-tui bottom_pane::chat_composer::tests::deleting_first_text_element_renumbers_following_text_element
        PASS [   0.069s] codex-tui bottom_pane::chat_composer::tests::disable_paste_burst_flushes_pending_first_char_and_inserts_immediately
        PASS [   0.079s] codex-tui bottom_pane::chat_composer::tests::duplicate_image_placeholders_get_suffix
        PASS [   0.079s] codex-tui bottom_pane::chat_composer::tests::edit_clears_pending_paste
        PASS [   0.080s] codex-tui bottom_pane::chat_composer::tests::empty_enter_returns_none
        PASS [   0.081s] codex-tui bottom_pane::chat_composer::tests::empty_vim_insert_escape_enters_normal_without_esc_hint
        PASS [   0.080s] codex-tui bottom_pane::chat_composer::tests::enter_queues_when_queue_submissions_is_enabled
        PASS [   0.076s] codex-tui bottom_pane::chat_composer::tests::enter_submits_when_unified_mention_popup_has_no_selection
        PASS [   0.105s] codex-tui bottom_pane::chat_composer::tests::enter_submits_when_file_popup_has_no_selection
        PASS [   0.075s] codex-tui bottom_pane::chat_composer::tests::esc_switches_vim_insert_to_normal
        PASS [   0.106s] codex-tui bottom_pane::chat_composer::tests::esc_exits_empty_shell_mode
        PASS [   0.091s] codex-tui bottom_pane::chat_composer::tests::esc_hint_stays_hidden_with_draft_content
        PASS [   0.082s] codex-tui bottom_pane::chat_composer::tests::file_completion_preserves_large_paste_placeholder_elements
        PASS [   0.827s] codex-tui bottom_pane::approval_overlay::tests::permissions_prompt_snapshot
        PASS [   0.117s] codex-tui bottom_pane::chat_composer::tests::esc_keeps_shell_mode_when_paste_burst_flushes_pending_text
        LEAK [   3.413s] codex-tui app::tests::update_memory_settings_updates_current_thread_memory_mode
        PASS [   0.103s] codex-tui bottom_pane::chat_composer::tests::footer_flash_expires_and_falls_back_to_hint_override
        PASS [   0.123s] codex-tui bottom_pane::chat_composer::tests::footer_flash_overrides_footer_hint_override
        PASS [   0.979s] codex-tui bottom_pane::approval_overlay::tests::network_exec_prompt_title_includes_host
        PASS [   0.111s] codex-tui bottom_pane::chat_composer::tests::handle_paste_small_inserts_text
        PASS [   0.122s] codex-tui bottom_pane::chat_composer::tests::footer_hint_row_is_separated_from_composer
        PASS [   0.076s] codex-tui bottom_pane::chat_composer::tests::history_navigation_restores_remote_only_submissions
        PASS [   0.084s] codex-tui bottom_pane::chat_composer::tests::history_navigation_restores_remote_and_local_image_attachments
        PASS [   0.158s] codex-tui bottom_pane::chat_composer::tests::handle_paste_large_uses_placeholder_and_replaces_on_submit
        PASS [   0.058s] codex-tui bottom_pane::chat_composer::tests::image_placeholder_backspace_behaves_like_text_placeholder
        PASS [   0.069s] codex-tui bottom_pane::chat_composer::tests::inline_slash_command_can_be_recalled_after_recording_pending_history
        PASS [   0.070s] codex-tui bottom_pane::chat_composer::tests::inline_slash_command_dispatch_resets_vim_mode_to_normal
        PASS [   0.060s] codex-tui bottom_pane::chat_composer::tests::input_disabled_ignores_keypresses_and_hides_cursor
        PASS [   0.062s] codex-tui bottom_pane::chat_composer::tests::kill_buffer_persists_after_slash_command_dispatch
        PASS [   0.044s] codex-tui bottom_pane::chat_composer::tests::kill_buffer_persists_after_submit
        PASS [   0.050s] codex-tui bottom_pane::chat_composer::tests::large_paste_numbering_continues_with_same_length_placeholder
        PASS [   0.197s] codex-tui bottom_pane::chat_composer::tests::history_navigation_from_start_of_bang_command_recalls_older_entry
        PASS [   0.186s] codex-tui bottom_pane::chat_composer::tests::history_navigation_leaves_cursor_at_end_of_line
        PASS [   0.051s] codex-tui bottom_pane::chat_composer::tests::large_paste_numbering_reuses_after_all_deleted
        PASS [   0.051s] codex-tui bottom_pane::chat_composer::tests::large_paste_numbering_reuses_after_ctrl_c_clear
        PASS [   0.050s] codex-tui bottom_pane::chat_composer::tests::large_paste_with_leading_whitespace_trims_and_shifts_elements
        PASS [   0.061s] codex-tui bottom_pane::chat_composer::tests::large_paste_preserves_image_text_elements_on_submit
        PASS [   0.068s] codex-tui bottom_pane::chat_composer::tests::mention_items_show_plugin_owned_skill_and_app_duplicates
        PASS [   0.065s] codex-tui bottom_pane::chat_composer::tests::non_ascii_char_inserts_immediately_without_burst_state
        PASS [   0.067s] codex-tui bottom_pane::chat_composer::tests::non_char_key_flushes_active_burst_before_input
        PASS [   0.129s] codex-tui bottom_pane::chat_composer::tests::non_ascii_burst_buffers_enter_and_flushes_multiline
        PASS [   0.132s] codex-tui bottom_pane::chat_composer::tests::non_ascii_burst_buffers_large_multiline_mixed_ascii_and_unicode
        PASS [   0.051s] codex-tui bottom_pane::chat_composer::tests::pasted_crlf_normalizes_newlines_for_elements
        PASS [   0.131s] codex-tui bottom_pane::chat_composer::tests::non_ascii_burst_preserves_ideographic_space_and_ascii
        PASS [   0.068s] codex-tui bottom_pane::chat_composer::tests::pasting_filepath_attaches_image
        PASS [   0.061s] codex-tui bottom_pane::chat_composer::tests::pending_first_ascii_char_flushes_as_typed
        PASS [   0.073s] codex-tui bottom_pane::chat_composer::tests::plugin_at_mentions_use_plugin_accent_style
        PASS [   0.057s] codex-tui bottom_pane::chat_composer::tests::popup_selected_slash_command_records_canonical_command_history
        PASS [   0.063s] codex-tui bottom_pane::chat_composer::tests::prepare_submission_keeps_remote_offset_local_placeholder_numbering
        PASS [   0.057s] codex-tui bottom_pane::chat_composer::tests::prepare_submission_with_only_remote_images_returns_empty_text
        PASS [   0.050s] codex-tui bottom_pane::chat_composer::tests::queued_submission_flushes_ascii_burst_instead_of_inserting_newline
        PASS [   0.121s] codex-tui bottom_pane::chat_composer::tests::question_mark_does_not_toggle_during_paste_burst
        PASS [   0.048s] codex-tui bottom_pane::chat_composer::tests::recalled_plugin_at_mentions_keep_plugin_accent_style
        PASS [   0.137s] codex-tui bottom_pane::chat_composer::tests::question_mark_only_toggles_on_first_char
        PASS [   0.062s] codex-tui bottom_pane::chat_composer::tests::remapped_history_search_does_not_fall_back_to_ctrl_r
        PASS [   0.054s] codex-tui bottom_pane::chat_composer::tests::remapped_queue_does_not_fall_back_to_tab
        PASS [   0.108s] codex-tui bottom_pane::chat_composer::tests::remapped_editor_history_navigation_does_not_fall_back_to_up
        PASS [   0.052s] codex-tui bottom_pane::chat_composer::tests::remapped_submit_does_not_fall_back_to_enter
        PASS [   0.052s] codex-tui bottom_pane::chat_composer::tests::remote_images_do_not_modify_textarea_text_or_elements
        PASS [   0.137s] codex-tui bottom_pane::chat_composer::tests::remapped_vim_normal_history_navigation_does_not_fall_back_to_j_k
        PASS [   0.046s] codex-tui bottom_pane::chat_composer::tests::remove_recording_meter_placeholder_clears_placeholder_text
        PASS [   0.816s] codex-tui bottom_pane::chat_composer::tests::footer_mode_snapshots
        PASS [   0.715s] codex-tui bottom_pane::chat_composer::tests::image_placeholder_snapshots
        PASS [   0.888s] codex-tui bottom_pane::chat_composer::tests::footer_collapse_snapshots
        PASS [   0.062s] codex-tui bottom_pane::chat_composer::tests::restored_bound_at_mentions_do_not_open_mention_popup
        PASS [   0.064s] codex-tui bottom_pane::chat_composer::tests::set_connector_mentions_excludes_disabled_apps_from_mention_popup
        PASS [   0.069s] codex-tui bottom_pane::chat_composer::tests::set_connector_mentions_refreshes_open_mention_popup
        PASS [   0.695s] codex-tui bottom_pane::chat_composer::tests::mention_popup_type_prefixes_snapshot
        PASS [   0.065s] codex-tui bottom_pane::chat_composer::tests::set_connector_mentions_skips_disabled_connectors
        PASS [   0.121s] codex-tui bottom_pane::chat_composer::tests::service_tier_slash_command_dispatches_from_catalog_name
        PASS [   0.063s] codex-tui bottom_pane::chat_composer::tests::set_plugin_mentions_refreshes_open_mention_popup
        PASS [   0.068s] codex-tui bottom_pane::chat_composer::tests::set_skill_mentions_refreshes_open_mention_popup
        PASS [   0.716s] codex-tui bottom_pane::chat_composer::tests::oversized_queued_submission_reports_error_and_restores_draft
        PASS [   0.069s] codex-tui bottom_pane::chat_composer::tests::set_text_content_rebinds_at_mentions_after_email_substrings
        PASS [   0.717s] codex-tui bottom_pane::chat_composer::tests::oversized_submit_reports_error_and_restores_draft
        PASS [   0.088s] codex-tui bottom_pane::chat_composer::tests::set_text_content_rebinds_at_mentions_after_punctuation
        PASS [   0.093s] codex-tui bottom_pane::chat_composer::tests::set_text_content_reattaches_images_without_placeholder_metadata
        PASS [   0.082s] codex-tui bottom_pane::chat_composer::tests::set_text_content_rebinds_at_sigiled_mentions
        PASS [   0.102s] codex-tui bottom_pane::chat_composer::tests::set_text_content_rebinds_both_sigil_forms
        PASS [   0.066s] codex-tui bottom_pane::chat_composer::tests::shell_command_cursor_uses_absorbed_prefix
        PASS [   0.099s] codex-tui bottom_pane::chat_composer::tests::set_text_content_rebinds_matching_sigil_only
        PASS [   0.087s] codex-tui bottom_pane::chat_composer::tests::shell_command_can_be_typed_after_vim_normal_bang
        PASS [   0.071s] codex-tui bottom_pane::chat_composer::tests::shell_command_uses_shell_accent_style
        PASS [   0.056s] codex-tui bottom_pane::chat_composer::tests::shift_question_mark_toggles_shortcut_overlay_when_empty
        PASS [   0.059s] codex-tui bottom_pane::chat_composer::tests::shortcut_overlay_persists_while_task_running
        PASS [   0.742s] codex-tui bottom_pane::chat_composer::tests::plugin_at_mentions_render_with_plugin_accent_snapshot
        PASS [   0.054s] codex-tui bottom_pane::chat_composer::tests::slash_command_disabled_while_task_running_keeps_text
        PASS [   0.745s] codex-tui bottom_pane::chat_composer::tests::plugin_mention_popup_snapshot
        PASS [   0.072s] codex-tui bottom_pane::chat_composer::tests::slash_command_can_be_typed_and_dispatched_after_vim_normal_slash
        PASS [   0.067s] codex-tui bottom_pane::chat_composer::tests::slash_context_enter_ignores_paste_burst_enter_suppression
        PASS [   0.078s] codex-tui bottom_pane::chat_composer::tests::slash_key_completes_selected_slash_command_as_text
        PASS [   0.068s] codex-tui bottom_pane::chat_composer::tests::slash_opens_command_popup_in_vim_normal_mode
        PASS [   0.052s] codex-tui bottom_pane::chat_composer::tests::slash_path_input_submits_without_command_error
        PASS [   0.123s] codex-tui bottom_pane::chat_composer::tests::slash_command_elementizes_on_space
        PASS [   0.117s] codex-tui bottom_pane::chat_composer::tests::slash_init_dispatches_command_and_does_not_submit_literal_text
        PASS [   0.148s] codex-tui bottom_pane::chat_composer::tests::slash_command_element_removed_when_not_at_start
        PASS [   0.127s] codex-tui bottom_pane::chat_composer::tests::slash_command_elementizes_only_known_commands
        PASS [   0.044s] codex-tui bottom_pane::chat_composer::tests::slash_popup_activated_for_bare_slash_and_valid_prefixes
        PASS [   0.098s] codex-tui bottom_pane::chat_composer::tests::slash_plan_args_preserve_text_elements
        PASS [   0.142s] codex-tui bottom_pane::chat_composer::tests::slash_mention_dispatches_command_and_inserts_at
        PASS [   0.086s] codex-tui bottom_pane::chat_composer::tests::slash_popup_btw_for_bt_logic
        PASS [   0.062s] codex-tui bottom_pane::chat_composer::tests::slash_popup_not_activated_for_slash_space_text_history_like_input
        PASS [   0.084s] codex-tui bottom_pane::chat_composer::tests::slash_popup_model_first_for_mo_logic
        PASS [   0.096s] codex-tui bottom_pane::chat_composer::tests::slash_popup_pets_for_pet_logic
        PASS [   0.094s] codex-tui bottom_pane::chat_composer::tests::slash_popup_resume_for_res_logic
        PASS [   0.086s] codex-tui bottom_pane::chat_composer::tests::slash_popup_side_for_si_logic
  TRY 1 FAIL [   0.062s] codex-tui bottom_pane::chat_composer::tests::slash_tab_completion_moves_cursor_to_end
  stdout ---

    running 1 test
    test bottom_pane::chat_composer::tests::slash_tab_completion_moves_cursor_to_end ... FAILED

    failures:

    failures:
        bottom_pane::chat_composer::tests::slash_tab_completion_moves_cursor_to_end

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.02s

  stderr ---

    thread 'bottom_pane::chat_composer::tests::slash_tab_completion_moves_cursor_to_end' (25724) panicked at tui\src\bottom_pane\chat_composer.rs:8620:9:
    assertion failed: `(left == right)`

    Diff < left / right > :
    </cs
    >/compact


    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.711s] codex-tui bottom_pane::chat_composer::tests::remote_image_rows_snapshots
        PASS [   0.080s] codex-tui bottom_pane::chat_composer::tests::slash_tab_then_enter_dispatches_builtin_command
        PASS [   0.085s] codex-tui bottom_pane::chat_composer::tests::slash_tab_completion_wins_over_queueing_while_task_running
        PASS [   0.051s] codex-tui bottom_pane::chat_composer::tests::slash_with_leading_space_submits_as_text
  TRY 2 FAIL [   0.073s] codex-tui bottom_pane::chat_composer::tests::slash_tab_completion_moves_cursor_to_end
  stdout ---

    running 1 test
    test bottom_pane::chat_composer::tests::slash_tab_completion_moves_cursor_to_end ... FAILED

    failures:

    failures:
        bottom_pane::chat_composer::tests::slash_tab_completion_moves_cursor_to_end

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.02s

  stderr ---

    thread 'bottom_pane::chat_composer::tests::slash_tab_completion_moves_cursor_to_end' (18748) panicked at tui\src\bottom_pane\chat_composer.rs:8620:9:
    assertion failed: `(left == right)`

    Diff < left / right > :
    </cs
    >/compact


    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.053s] codex-tui bottom_pane::chat_composer::tests::status_line_hyperlink_marks_pr_number_cells
        PASS [   0.059s] codex-tui bottom_pane::chat_composer::tests::submit_at_character_limit_succeeds
        PASS [   0.055s] codex-tui bottom_pane::chat_composer::tests::submit_captures_recent_mention_bindings_before_clearing_textarea
        PASS [   0.084s] codex-tui bottom_pane::chat_composer::tests::suppressed_submission_restores_pending_paste_payload
        PASS [   0.070s] codex-tui bottom_pane::chat_composer::tests::tab_queues_bang_shell_prompts_while_task_running_without_execution
        PASS [   0.066s] codex-tui bottom_pane::chat_composer::tests::tab_queues_leading_space_slash_as_plain_text_while_task_running
        PASS [   0.102s] codex-tui bottom_pane::chat_composer::tests::tab_does_not_submit_for_bang_shell_command
        PASS [   0.056s] codex-tui bottom_pane::chat_composer::tests::tab_queues_slash_led_prompts_while_task_running_without_validation
        PASS [   0.058s] codex-tui bottom_pane::chat_composer::tests::test_current_at_token_allows_file_queries_with_second_at
        PASS [   0.082s] codex-tui bottom_pane::chat_composer::tests::tab_submits_when_no_task_running
        PASS [   0.067s] codex-tui bottom_pane::chat_composer::tests::test_current_at_token_basic_cases
        PASS [   0.060s] codex-tui bottom_pane::chat_composer::tests::test_current_at_token_cursor_positions
        PASS [   0.672s] codex-tui bottom_pane::chat_composer::tests::shutdown_in_progress_disables_input_and_uses_hint_without_footer
        PASS [   0.062s] codex-tui bottom_pane::chat_composer::tests::test_current_at_token_ignores_mid_word_at
        PASS [   0.061s] codex-tui bottom_pane::chat_composer::tests::test_current_at_token_tracks_tokens_with_second_at
        PASS [   0.061s] codex-tui bottom_pane::chat_composer::tests::test_current_at_token_whitespace_boundaries
        PASS [   0.061s] codex-tui bottom_pane::chat_composer::tests::test_multiple_pastes_submission
        PASS [   0.059s] codex-tui bottom_pane::chat_composer::tests::test_partial_placeholder_deletion
        PASS [   0.075s] codex-tui bottom_pane::chat_composer::tests::test_placeholder_deletion
        PASS [   0.068s] codex-tui bottom_pane::chat_composer::tests::vim_insert_uses_bar_cursor_style
        PASS [   0.064s] codex-tui bottom_pane::chat_composer::tests::vim_mode_resets_to_normal_after_queued_submission
        PASS [   0.063s] codex-tui bottom_pane::chat_composer::tests::vim_mode_resets_to_normal_after_submission
        PASS [   0.059s] codex-tui bottom_pane::chat_composer::tests::vim_mode_stays_insert_after_suppressed_submission
        PASS [   0.706s] codex-tui bottom_pane::chat_composer::tests::slash_popup_archive_for_ar_ui
        PASS [   0.069s] codex-tui bottom_pane::chat_composer::tests::vim_normal_j_k_fall_back_to_multiline_cursor_movement
        PASS [   0.053s] codex-tui bottom_pane::chat_composer::tests::vim_normal_operator_pending_consumes_submit_key
        PASS [   0.054s] codex-tui bottom_pane::chat_composer_history::tests::duplicate_submissions_are_not_recorded
        PASS [   0.742s] codex-tui bottom_pane::chat_composer::tests::slash_popup_model_first_for_mo_ui
        PASS [   0.148s] codex-tui bottom_pane::chat_composer::tests::vim_normal_history_navigation_from_start_of_bang_command_recalls_older_entry
        PASS [   0.779s] codex-tui bottom_pane::chat_composer::tests::slash_popup_btw_for_bt_ui
        PASS [   0.163s] codex-tui bottom_pane::chat_composer::tests::vim_normal_j_k_navigate_history_at_history_boundaries
        PASS [   0.065s] codex-tui bottom_pane::chat_composer_history::tests::navigation_with_async_fetch
        PASS [   0.067s] codex-tui bottom_pane::chat_composer_history::tests::persistent_restore_gates_at_mentions
        PASS [   0.765s] codex-tui bottom_pane::chat_composer::tests::slash_popup_pets_for_pet_ui
        PASS [   0.068s] codex-tui bottom_pane::chat_composer_history::tests::repeated_boundary_search_does_not_refetch_persistent_history
        PASS [   0.163s] codex-tui bottom_pane::chat_composer::tests::vim_normal_operator_motion_does_not_navigate_history
        PASS [   0.752s] codex-tui bottom_pane::chat_composer::tests::slash_popup_side_for_si_ui
        PASS [   0.065s] codex-tui bottom_pane::chat_composer_history::tests::search_fetches_persistent_history_until_match
        PASS [   0.066s] codex-tui bottom_pane::chat_composer_history::tests::search_is_case_insensitive_and_empty_query_finds_latest
        PASS [   0.097s] codex-tui bottom_pane::chat_composer_history::tests::reset_navigation_resets_cursor
        PASS [   0.788s] codex-tui bottom_pane::chat_composer::tests::slash_popup_resume_for_res_ui
        PASS [   0.058s] codex-tui bottom_pane::chat_composer_history::tests::search_matches_local_history_and_stops_at_boundaries
        PASS [   0.063s] codex-tui bottom_pane::chat_composer_history::tests::search_skips_duplicate_persistent_matches
        PASS [   0.069s] codex-tui bottom_pane::chat_composer_history::tests::search_skips_duplicate_local_matches
        PASS [   0.069s] codex-tui bottom_pane::chat_composer_history::tests::should_handle_navigation_when_cursor_is_at_line_boundaries
        PASS [   0.054s] codex-tui bottom_pane::command_popup::tests::btw_hidden_in_empty_filter_but_shown_for_prefix
        PASS [   0.060s] codex-tui bottom_pane::command_popup::tests::debug_commands_are_hidden_from_popup
        PASS [   0.063s] codex-tui bottom_pane::command_popup::tests::filter_includes_init_when_typing_prefix
        PASS [   0.071s] codex-tui bottom_pane::command_popup::tests::filtered_commands_keep_presentation_order_for_prefix
        PASS [   0.076s] codex-tui bottom_pane::command_popup::tests::model_is_first_suggestion_for_mo
        PASS [   0.075s] codex-tui bottom_pane::command_popup::tests::personality_command_hidden_when_disabled
        PASS [   0.077s] codex-tui bottom_pane::command_popup::tests::personality_command_visible_when_enabled
        PASS [   0.086s] codex-tui bottom_pane::command_popup::tests::plan_command_hidden_when_collaboration_modes_disabled
        PASS [   0.077s] codex-tui bottom_pane::command_popup::tests::plan_command_visible_when_collaboration_modes_enabled
        PASS [   0.073s] codex-tui bottom_pane::command_popup::tests::prefix_filter_limits_matches_for_ac
        PASS [   0.067s] codex-tui bottom_pane::command_popup::tests::quit_hidden_in_empty_filter_but_shown_for_prefix
        PASS [   0.066s] codex-tui bottom_pane::command_popup::tests::selecting_init_by_exact_match
        PASS [   0.069s] codex-tui bottom_pane::command_popup::tests::service_tier_command_uses_catalog_name_and_description
        PASS [   0.073s] codex-tui bottom_pane::command_popup::tests::settings_command_hidden_when_audio_device_selection_is_disabled
        PASS [   0.072s] codex-tui bottom_pane::custom_prompt_view::tests::delayed_enter_after_typing_submits
        PASS [   0.071s] codex-tui bottom_pane::custom_prompt_view::tests::paste_burst_newline_after_tab_does_not_submit
        PASS [   0.081s] codex-tui bottom_pane::custom_prompt_view::tests::paste_burst_newline_does_not_submit_short_first_line
        PASS [   0.073s] codex-tui bottom_pane::feedback_view::tests::feedback_success_cell_matches_employee_bug_copy
        PASS [   0.066s] codex-tui bottom_pane::feedback_view::tests::feedback_success_cell_matches_external_bug_copy
        PASS [   0.068s] codex-tui bottom_pane::feedback_view::tests::feedback_success_cell_matches_good_result_copy
        PASS [   0.071s] codex-tui bottom_pane::feedback_view::tests::feedback_success_cell_uses_issue_links_for_remaining_categories
        PASS [   0.740s] codex-tui bottom_pane::chat_composer::tests::ui_snapshots
        PASS [   0.045s] codex-tui bottom_pane::feedback_view::tests::issue_url_available_for_bug_bad_result_safety_check_and_other
        PASS [   0.051s] codex-tui bottom_pane::feedback_view::tests::should_show_feedback_connectivity_details_only_for_non_good_result_with_diagnostics
        PASS [   0.063s] codex-tui bottom_pane::feedback_view::tests::submit_feedback_emits_submit_event_with_trimmed_note
        PASS [   0.057s] codex-tui bottom_pane::feedback_view::tests::submit_feedback_omits_empty_note
        PASS [   0.717s] codex-tui bottom_pane::command_popup::tests::app_command_popup_snapshot
        PASS [   0.694s] codex-tui bottom_pane::command_popup::tests::changing_filter_resets_selection_after_scrolling
        PASS [   0.077s] codex-tui bottom_pane::file_search_popup::tests::set_matches_keeps_only_the_first_page_of_results
        PASS [   0.067s] codex-tui bottom_pane::footer::tests::paste_image_shortcut_prefers_ctrl_alt_v_under_wsl
        PASS [   0.080s] codex-tui bottom_pane::footer::tests::footer_status_line_truncates_to_keep_mode_indicator
        PASS [   0.058s] codex-tui bottom_pane::hooks_browser_view::tests::escape_returns_to_the_selected_event
        PASS [   0.064s] codex-tui bottom_pane::hooks_browser_view::tests::esc_routes_through_the_view
        PASS [   0.752s] codex-tui bottom_pane::feedback_view::tests::feedback_upload_consent_lists_doctor_report
        PASS [   0.112s] codex-tui bottom_pane::hooks_browser_view::tests::managed_hooks_count_as_active
        PASS [   0.768s] codex-tui bottom_pane::feedback_view::tests::feedback_view_bad_result
        PASS [   0.758s] codex-tui bottom_pane::feedback_view::tests::feedback_view_bug
        PASS [   0.750s] codex-tui bottom_pane::feedback_view::tests::feedback_view_other
        PASS [   0.791s] codex-tui bottom_pane::feedback_view::tests::feedback_upload_consent_lists_windows_sandbox_log_when_included
        PASS [   0.776s] codex-tui bottom_pane::feedback_view::tests::feedback_view_good_result
        PASS [   0.781s] codex-tui bottom_pane::feedback_view::tests::feedback_view_with_connectivity_diagnostics
        PASS [   0.840s] codex-tui bottom_pane::feedback_view::tests::feedback_view_safety_check
        PASS [   0.756s] codex-tui bottom_pane::footer::tests::footer_snapshots
        PASS [   0.783s] codex-tui bottom_pane::hooks_browser_view::tests::renders_command_details_with_three_line_cap
        PASS [   0.711s] codex-tui bottom_pane::hooks_browser_view::tests::renders_empty_handler_browser_message
        PASS [   0.060s] codex-tui bottom_pane::hooks_browser_view::tests::review_needed_event_is_selected_by_default
        PASS [   0.061s] codex-tui bottom_pane::hooks_browser_view::tests::review_needed_handler_header_uses_warning_color
        PASS [   0.755s] codex-tui bottom_pane::hooks_browser_view::tests::renders_event_browser
        PASS [   0.758s] codex-tui bottom_pane::hooks_browser_view::tests::renders_handler_browser_with_details
        PASS [   0.755s] codex-tui bottom_pane::hooks_browser_view::tests::renders_review_needed_handler
        PASS [   0.767s] codex-tui bottom_pane::hooks_browser_view::tests::renders_event_browser_with_issues
        PASS [   0.061s] codex-tui bottom_pane::hooks_browser_view::tests::review_needed_handler_rows_use_warning_color
        PASS [   0.778s] codex-tui bottom_pane::hooks_browser_view::tests::renders_managed_handler_without_toggle_hint
        PASS [   0.727s] codex-tui bottom_pane::hooks_browser_view::tests::renders_scrolled_handler_window
        PASS [   0.055s] codex-tui bottom_pane::hooks_browser_view::tests::review_needed_hooks_are_not_active
        PASS [   0.044s] codex-tui bottom_pane::hooks_browser_view::tests::selected_event_rows_use_the_shared_accent_style
        PASS [   0.801s] codex-tui bottom_pane::hooks_browser_view::tests::renders_event_browser_with_review_column_when_needed
        PASS [   0.045s] codex-tui bottom_pane::hooks_browser_view::tests::space_does_not_toggle_managed_handler
        PASS [   0.045s] codex-tui bottom_pane::hooks_browser_view::tests::toggle_keys_toggle_unmanaged_handler
        PASS [   0.739s] codex-tui bottom_pane::hooks_browser_view::tests::renders_selected_managed_handler
        PASS [   0.061s] codex-tui bottom_pane::hooks_browser_view::tests::trust_key_on_event_page_trusts_all_review_needed_hooks
        PASS [   0.066s] codex-tui bottom_pane::hooks_browser_view::tests::trust_key_preserves_disabled_modified_handler
        PASS [   0.070s] codex-tui bottom_pane::hooks_browser_view::tests::trust_key_trusts_review_needed_handler_without_changing_enablement
        PASS [   0.073s] codex-tui bottom_pane::list_selection_view::tests::c0_ctrl_n_respects_unbound_list_move_down
        PASS [   0.072s] codex-tui bottom_pane::list_selection_view::tests::c0_ctrl_p_respects_remapped_list_move_down
        PASS [   0.085s] codex-tui bottom_pane::list_selection_view::tests::auto_all_rows_col_width_does_not_shift_when_scrolling
        PASS [   0.078s] codex-tui bottom_pane::list_selection_view::tests::c0_ctrl_p_respects_unbound_list_move_up
        PASS [   0.080s] codex-tui bottom_pane::list_selection_view::tests::enter_with_no_matches_triggers_cancel_callback
        PASS [   0.095s] codex-tui bottom_pane::list_selection_view::tests::disabled_current_rows_skip_default_selection_and_number_shortcuts
        PASS [   0.079s] codex-tui bottom_pane::list_selection_view::tests::move_down_without_selection_change_does_not_fire_callback
        PASS [   0.091s] codex-tui bottom_pane::list_selection_view::tests::fixed_col_width_is_30_70_and_does_not_shift_when_scrolling
        PASS [   0.093s] codex-tui bottom_pane::list_selection_view::tests::name_column_width_override_moves_description_column_right
        PASS [   0.097s] codex-tui bottom_pane::list_selection_view::tests::narrow_width_keeps_all_rows_visible
        PASS [   0.080s] codex-tui bottom_pane::list_selection_view::tests::page_and_jump_navigation_skip_trailing_disabled_rows_without_wrapping
        PASS [   0.084s] codex-tui bottom_pane::list_selection_view::tests::page_and_jump_navigation_use_list_keymap
        PASS [   0.090s] codex-tui bottom_pane::list_selection_view::tests::paste_appends_to_search_query_and_filters_items
        PASS [   0.114s] codex-tui bottom_pane::list_selection_view::tests::preserve_side_content_bg_keeps_rendered_background_colors
        PASS [   0.082s] codex-tui bottom_pane::list_selection_view::tests::renders_search_query_line_when_enabled
        PASS [   0.100s] codex-tui bottom_pane::list_selection_view::tests::side_content_clearing_handles_non_zero_buffer_origin
        PASS [   0.071s] codex-tui bottom_pane::list_selection_view::tests::side_layout_width_half_falls_back_when_list_would_be_too_narrow
        PASS [   0.094s] codex-tui bottom_pane::list_selection_view::tests::side_content_clearing_resets_symbols_and_style
        PASS [   0.080s] codex-tui bottom_pane::list_selection_view::tests::side_layout_width_half_uses_exact_split
        PASS [   0.106s] codex-tui bottom_pane::list_selection_view::tests::single_line_row_display_truncates_instead_of_wrapping
        PASS [   0.080s] codex-tui bottom_pane::list_selection_view::tests::space_appends_to_active_search_instead_of_toggling_selected_item
        PASS [   0.072s] codex-tui bottom_pane::list_selection_view::tests::stacked_side_content_is_used_when_side_by_side_does_not_fit
        PASS [   0.069s] codex-tui bottom_pane::list_selection_view::tests::switching_tabs_changes_visible_items_and_clears_search
        PASS [   0.051s] codex-tui bottom_pane::list_selection_view::tests::tabbed_view_preserves_current_row_on_initial_selection_and_tab_switch
        PASS [   0.726s] codex-tui bottom_pane::hooks_browser_view::tests::renders_untrusted_enabled_handler_as_inactive
        PASS [   0.049s] codex-tui bottom_pane::list_selection_view::tests::whitespace_only_paste_is_ignored
        PASS [   0.066s] codex-tui bottom_pane::list_selection_view::tests::theme_picker_enables_side_content_background_preservation
        PASS [   0.063s] codex-tui bottom_pane::list_selection_view::tests::wraps_long_option_without_overflowing_columns
        PASS [   0.248s] codex-tui bottom_pane::list_selection_view::tests::theme_picker_subtitle_uses_fallback_text_in_94x35_terminal
        PASS [   0.738s] codex-tui bottom_pane::list_selection_view::tests::renders_blank_line_between_subtitle_and_items
        PASS [   0.757s] codex-tui bottom_pane::list_selection_view::tests::renders_blank_line_between_title_and_items_without_subtitle
        PASS [   0.440s] codex-tui bottom_pane::list_selection_view::tests::width_changes_do_not_hide_rows
        PASS [   0.706s] codex-tui bottom_pane::list_selection_view::tests::snapshot_footer_note_wraps
        PASS [   0.802s] codex-tui bottom_pane::list_selection_view::tests::snapshot_auto_all_rows_col_width_mode_scroll_behavior
        PASS [   0.730s] codex-tui bottom_pane::list_selection_view::tests::snapshot_model_picker_width_80
        PASS [   0.084s] codex-tui bottom_pane::mcp_server_elicitation::tests::ctrl_c_cancels_elicitation
        PASS [   0.738s] codex-tui bottom_pane::list_selection_view::tests::snapshot_narrow_width_preserves_third_option
        PASS [   0.788s] codex-tui bottom_pane::list_selection_view::tests::snapshot_auto_visible_col_width_mode_scroll_behavior
        PASS [   0.048s] codex-tui bottom_pane::mcp_server_elicitation::tests::empty_object_schema_uses_approval_actions
        PASS [   0.790s] codex-tui bottom_pane::list_selection_view::tests::snapshot_fixed_col_width_mode_scroll_behavior
        PASS [   0.044s] codex-tui bottom_pane::mcp_server_elicitation::tests::empty_tool_approval_schema_session_choice_sets_persist_meta
        PASS [   0.060s] codex-tui bottom_pane::mcp_server_elicitation::tests::empty_tool_approval_schema_always_allow_sets_persist_meta
        PASS [   0.068s] codex-tui bottom_pane::mcp_server_elicitation::tests::empty_tool_approval_schema_uses_approval_actions
        PASS [   0.073s] codex-tui bottom_pane::mcp_server_elicitation::tests::horizontal_list_keys_move_between_select_fields
        PASS [   0.099s] codex-tui bottom_pane::mcp_server_elicitation::tests::parses_boolean_form_request
        PASS [   0.089s] codex-tui bottom_pane::mcp_server_elicitation::tests::plugin_tool_suggestion_meta_without_install_url_is_parsed_into_request_payload
        PASS [   0.071s] codex-tui bottom_pane::mcp_server_elicitation::tests::resolved_request_dismisses_overlay_without_emitting_events
        PASS [   0.073s] codex-tui bottom_pane::mcp_server_elicitation::tests::submit_sends_accept_with_typed_content
        PASS [   0.093s] codex-tui bottom_pane::mcp_server_elicitation::tests::queues_requests_fifo
        PASS [   0.075s] codex-tui bottom_pane::mcp_server_elicitation::tests::tool_approval_display_params_prefer_explicit_display_order
        PASS [   0.086s] codex-tui bottom_pane::mcp_server_elicitation::tests::tool_suggestion_meta_is_parsed_into_request_payload
        PASS [   0.082s] codex-tui bottom_pane::mcp_server_elicitation::tests::unsupported_numeric_form_falls_back
        PASS [   0.083s] codex-tui bottom_pane::mentions_v2::search_catalog::tests::plugin_mention_name_uses_display_segments_when_they_match_plugin_name
        PASS [   0.096s] codex-tui bottom_pane::mentions_v2::search_catalog::tests::plugin_mention_name_falls_back_to_title_cased_plugin_name
        PASS [   0.049s] codex-tui bottom_pane::multi_select_picker::tests::horizontal_list_keys_reorder_orderable_items
        PASS [   0.057s] codex-tui bottom_pane::multi_select_picker::tests::page_and_jump_navigation_use_list_keymap
        PASS [   0.735s] codex-tui bottom_pane::mcp_server_elicitation::tests::approval_form_tool_approval_snapshot
        PASS [   0.063s] codex-tui bottom_pane::multi_select_picker::tests::non_orderable_items_cannot_move_or_be_crossed
        PASS [   0.059s] codex-tui bottom_pane::multi_select_picker::tests::searchable_plain_j_updates_query_instead_of_navigating
        PASS [   0.059s] codex-tui bottom_pane::multi_select_picker::tests::section_break_after_item_renders_separator_row
        PASS [   0.055s] codex-tui bottom_pane::paste_burst::tests::ascii_first_char_is_held_then_flushes_as_typed
        PASS [   0.056s] codex-tui bottom_pane::paste_burst::tests::decide_begin_buffer_only_triggers_for_pastey_prefixes
        PASS [   0.056s] codex-tui bottom_pane::paste_burst::tests::ascii_two_fast_chars_start_buffer_from_pending_and_flush_as_paste
        PASS [   0.050s] codex-tui bottom_pane::paste_burst::tests::flush_before_modified_input_includes_pending_first_char
        PASS [   0.065s] codex-tui bottom_pane::pending_input_preview::tests::desired_height_empty
        PASS [   0.070s] codex-tui bottom_pane::paste_burst::tests::newline_suppression_window_outlives_buffer_flush
        PASS [   0.707s] codex-tui bottom_pane::mcp_server_elicitation::tests::approval_form_tool_approval_with_param_summary_snapshot
        PASS [   0.058s] codex-tui bottom_pane::pending_input_preview::tests::desired_height_one_message
        PASS [   0.058s] codex-tui bottom_pane::pending_input_preview::tests::long_url_like_message_does_not_expand_into_wrapped_ellipsis_rows
        PASS [   0.728s] codex-tui bottom_pane::mcp_server_elicitation::tests::approval_form_tool_approval_with_persist_options_snapshot
        PASS [   0.742s] codex-tui bottom_pane::mcp_server_elicitation::tests::boolean_form_snapshot
        PASS [   0.783s] codex-tui bottom_pane::mcp_server_elicitation::tests::message_only_form_snapshot
        PASS [   0.784s] codex-tui bottom_pane::mcp_server_elicitation::tests::message_only_form_with_persist_options_snapshot
        PASS [   0.058s] codex-tui bottom_pane::pending_thread_approvals::tests::desired_height_empty
        PASS [   0.716s] codex-tui bottom_pane::pending_input_preview::tests::render_more_than_three_messages
        PASS [   0.738s] codex-tui bottom_pane::pending_input_preview::tests::render_many_line_message
        PASS [   0.722s] codex-tui bottom_pane::pending_input_preview::tests::render_one_message
        PASS [   0.723s] codex-tui bottom_pane::pending_input_preview::tests::render_one_message_with_shift_left_binding
        PASS [   0.734s] codex-tui bottom_pane::pending_input_preview::tests::render_multiline_pending_steer_uses_single_prefix_and_truncates
        PASS [   0.728s] codex-tui bottom_pane::pending_input_preview::tests::render_one_pending_steer_with_remapped_interrupt_binding
        PASS [   0.753s] codex-tui bottom_pane::pending_input_preview::tests::render_one_pending_steer
        PASS [   0.070s] codex-tui bottom_pane::request_user_input::tests::backspace_on_empty_notes_closes_notes_ui
        PASS [   0.088s] codex-tui bottom_pane::request_user_input::tests::backspace_in_options_clears_selection
        PASS [   0.098s] codex-tui bottom_pane::request_user_input::tests::desired_height_keeps_spacers_and_preferred_options_visible
        PASS [   0.113s] codex-tui bottom_pane::request_user_input::tests::enter_commits_default_selection_on_last_option_question
        PASS [   0.100s] codex-tui bottom_pane::request_user_input::tests::enter_commits_default_selection_on_non_last_option_question
        PASS [   0.098s] codex-tui bottom_pane::request_user_input::tests::esc_drops_committed_answers
        PASS [   0.048s] codex-tui bottom_pane::request_user_input::tests::esc_in_notes_mode_clears_notes_and_hides_ui
        PASS [   0.062s] codex-tui bottom_pane::request_user_input::tests::esc_in_notes_mode_with_text_clears_notes_and_hides_ui
        PASS [   0.686s] codex-tui bottom_pane::pending_input_preview::tests::render_pending_steers_above_queued_messages
        PASS [   0.077s] codex-tui bottom_pane::request_user_input::tests::esc_in_notes_mode_without_options_interrupts
        PASS [   0.086s] codex-tui bottom_pane::request_user_input::tests::esc_in_options_mode_interrupts
        PASS [   0.081s] codex-tui bottom_pane::request_user_input::tests::footer_wraps_tips_without_splitting_individual_tips
        PASS [   0.053s] codex-tui bottom_pane::request_user_input::tests::freeform_draft_is_not_submitted_without_enter
        PASS [   0.079s] codex-tui bottom_pane::request_user_input::tests::freeform_commit_resets_when_draft_changes
        PASS [   0.680s] codex-tui bottom_pane::pending_input_preview::tests::render_two_messages
        PASS [   0.060s] codex-tui bottom_pane::request_user_input::tests::freeform_footer_shows_configured_submit_binding
        PASS [   0.062s] codex-tui bottom_pane::request_user_input::tests::freeform_enter_with_empty_text_is_unanswered
        PASS [   0.054s] codex-tui bottom_pane::request_user_input::tests::freeform_questions_submit_empty_when_empty
        PASS [   0.050s] codex-tui bottom_pane::request_user_input::tests::freeform_requires_enter_with_text_to_mark_answered
        PASS [   0.046s] codex-tui bottom_pane::request_user_input::tests::freeform_shows_ctrl_p_and_ctrl_n_question_navigation_tip
        PASS [   0.057s] codex-tui bottom_pane::request_user_input::tests::freeform_submit_binding_wins_over_question_navigation
        PASS [   0.070s] codex-tui bottom_pane::request_user_input::tests::freeform_shift_enter_inserts_newline_without_advancing
        PASS [   0.069s] codex-tui bottom_pane::request_user_input::tests::freeform_uses_configured_composer_submit_binding
        PASS [   0.054s] codex-tui bottom_pane::request_user_input::tests::h_l_move_between_questions_in_options
        PASS [   0.067s] codex-tui bottom_pane::request_user_input::tests::highlighted_option_questions_are_unanswered
        PASS [   0.070s] codex-tui bottom_pane::request_user_input::tests::horizontal_list_keys_move_between_questions_in_options
        PASS [   0.077s] codex-tui bottom_pane::request_user_input::tests::interrupt_discards_queued_requests_and_emits_interrupt
        PASS [   0.076s] codex-tui bottom_pane::request_user_input::tests::is_other_adds_none_of_the_above_and_submits_it
        PASS [   0.069s] codex-tui bottom_pane::request_user_input::tests::large_paste_is_preserved_when_switching_questions
        PASS [   0.067s] codex-tui bottom_pane::request_user_input::tests::layout_allocates_all_wrapped_options_when_space_allows
        PASS [   0.052s] codex-tui bottom_pane::request_user_input::tests::left_right_move_between_questions_in_options
        PASS [   0.054s] codex-tui bottom_pane::request_user_input::tests::notes_are_captured_for_selected_option
        PASS [   0.694s] codex-tui bottom_pane::pending_input_preview::tests::render_wrapped_message
        PASS [   0.069s] codex-tui bottom_pane::request_user_input::tests::notes_submission_commits_selected_option
        PASS [   0.075s] codex-tui bottom_pane::request_user_input::tests::number_keys_select_and_submit_options
        PASS [   0.069s] codex-tui bottom_pane::request_user_input::tests::options_notes_focus_hides_question_navigation_tip
        PASS [   0.076s] codex-tui bottom_pane::request_user_input::tests::options_can_submit_empty_when_unanswered
        PASS [   0.072s] codex-tui bottom_pane::request_user_input::tests::options_scroll_while_editing_notes
        PASS [   0.074s] codex-tui bottom_pane::request_user_input::tests::pending_paste_placeholder_survives_submission_and_back_navigation
        PASS [   0.061s] codex-tui bottom_pane::request_user_input::tests::queued_requests_are_fifo
        PASS [   0.694s] codex-tui bottom_pane::pending_thread_approvals::tests::render_multiple_threads_snapshot
        PASS [   0.724s] codex-tui bottom_pane::pending_thread_approvals::tests::render_single_thread_snapshot
        PASS [   0.726s] codex-tui bottom_pane::request_user_input::tests::request_user_input_freeform_remapped_submit_snapshot
        PASS [   0.780s] codex-tui bottom_pane::request_user_input::tests::request_user_input_freeform_remapped_interrupt_snapshot
        PASS [   0.791s] codex-tui bottom_pane::request_user_input::tests::request_user_input_footer_wrap_snapshot
        PASS [   0.760s] codex-tui bottom_pane::request_user_input::tests::request_user_input_hidden_options_footer_snapshot
        PASS [   0.770s] codex-tui bottom_pane::request_user_input::tests::request_user_input_long_option_text_snapshot
        PASS [   0.739s] codex-tui bottom_pane::request_user_input::tests::request_user_input_options_snapshot
        PASS [   0.781s] codex-tui bottom_pane::request_user_input::tests::request_user_input_multi_question_first_snapshot
        PASS [   0.816s] codex-tui bottom_pane::request_user_input::tests::request_user_input_freeform_snapshot
        PASS [   0.779s] codex-tui bottom_pane::request_user_input::tests::request_user_input_options_notes_visible_snapshot
        PASS [   0.042s] codex-tui bottom_pane::request_user_input::tests::request_user_input_uses_remapped_interrupt_binding_while_notes_are_visible
        PASS [   0.800s] codex-tui bottom_pane::request_user_input::tests::request_user_input_multi_question_last_snapshot
        PASS [   0.071s] codex-tui bottom_pane::request_user_input::tests::resolved_current_request_advances_to_next_same_turn_prompt
        PASS [   0.074s] codex-tui bottom_pane::request_user_input::tests::resolved_queued_request_removes_only_that_prompt
        PASS [   0.083s] codex-tui bottom_pane::request_user_input::tests::resolved_request_dismisses_overlay_without_emitting_events
        PASS [   0.074s] codex-tui bottom_pane::request_user_input::tests::selected_long_wrapped_option_stays_visible
        PASS [   0.085s] codex-tui bottom_pane::request_user_input::tests::skipped_option_questions_count_as_unanswered
        PASS [   0.082s] codex-tui bottom_pane::request_user_input::tests::switching_to_options_resets_notes_focus_when_notes_hidden
        PASS [   0.064s] codex-tui bottom_pane::request_user_input::tests::tab_in_notes_clears_notes_and_hides_ui
        PASS [   0.101s] codex-tui bottom_pane::request_user_input::tests::switching_from_freeform_with_text_resets_focus_and_keeps_last_option_empty
        PASS [   0.053s] codex-tui bottom_pane::request_user_input::tests::tab_opens_notes_when_option_selected
        PASS [   0.046s] codex-tui bottom_pane::request_user_input::tests::vim_keys_move_option_selection
        PASS [   0.055s] codex-tui bottom_pane::request_user_input::tests::typing_in_options_does_not_open_notes
        PASS [   0.059s] codex-tui bottom_pane::scroll_state::tests::page_and_jump_navigation_clamps
        PASS [   0.074s] codex-tui bottom_pane::scroll_state::tests::wrap_navigation_and_visibility
        PASS [   0.069s] codex-tui bottom_pane::selection_popup_common::tests::one_cell_width_falls_back_without_panic_for_wrapped_two_column_rows
        PASS [   0.083s] codex-tui bottom_pane::selection_popup_common::tests::selected_rows_use_the_shared_accent_style
        PASS [   0.082s] codex-tui bottom_pane::skill_popup::tests::display_name_match_sorting_beats_worse_secondary_search_term_matches
        PASS [   0.044s] codex-tui bottom_pane::skill_popup::tests::filtered_mentions_preserve_results_beyond_popup_height
        PASS [   0.740s] codex-tui bottom_pane::request_user_input::tests::request_user_input_scroll_options_snapshot
        PASS [   0.064s] codex-tui bottom_pane::skill_popup::tests::query_match_score_sorts_before_plugin_rank_bias
        PASS [   0.066s] codex-tui bottom_pane::skills_toggle_view::tests::footer_hint_uses_list_keymap_accept_and_cancel
        PASS [   0.058s] codex-tui bottom_pane::slash_commands::tests::clean_command_alias_resolves_for_dispatch
        PASS [   0.080s] codex-tui bottom_pane::slash_commands::tests::all_service_tiers_are_exposed_as_commands_after_model
        PASS [   0.075s] codex-tui bottom_pane::slash_commands::tests::clear_command_resolves_for_dispatch
        PASS [   0.057s] codex-tui bottom_pane::slash_commands::tests::goal_command_is_hidden_when_disabled
        PASS [   0.071s] codex-tui bottom_pane::slash_commands::tests::debug_command_still_resolves_for_dispatch
        PASS [   0.057s] codex-tui bottom_pane::slash_commands::tests::service_tier_commands_are_hidden_when_disabled
        PASS [   0.062s] codex-tui bottom_pane::slash_commands::tests::settings_command_is_hidden_when_realtime_is_disabled
        PASS [   0.083s] codex-tui bottom_pane::slash_commands::tests::realtime_command_is_hidden_when_realtime_is_disabled
        PASS [   0.083s] codex-tui bottom_pane::slash_commands::tests::settings_command_is_hidden_when_audio_device_selection_is_disabled
        PASS [   0.070s] codex-tui bottom_pane::slash_commands::tests::side_conversation_exact_lookup_still_resolves_service_tier_commands_for_dispatch_error
        PASS [   0.083s] codex-tui bottom_pane::slash_commands::tests::side_conversation_exact_lookup_still_resolves_hidden_commands_for_dispatch_error
        PASS [   0.059s] codex-tui bottom_pane::slash_commands::tests::side_conversation_hides_commands_without_side_flag
        PASS [   0.047s] codex-tui bottom_pane::slash_commands::tests::stop_command_resolves_for_dispatch
        PASS [   0.067s] codex-tui bottom_pane::status_line_setup::tests::context_remaining_is_selectable_id
        PASS [   0.069s] codex-tui bottom_pane::status_line_setup::tests::context_used_accepts_context_usage_legacy_id
        PASS [   0.074s] codex-tui bottom_pane::status_line_setup::tests::model_is_canonical_and_accepts_model_name_legacy_id
        PASS [   0.083s] codex-tui bottom_pane::status_line_setup::tests::git_summary_items_are_selectable_ids
        PASS [   0.073s] codex-tui bottom_pane::status_line_setup::tests::parse_status_line_items_accepts_title_only_variants
        PASS [   0.080s] codex-tui bottom_pane::status_line_setup::tests::preview_includes_thread_title
        PASS [   0.071s] codex-tui bottom_pane::status_line_setup::tests::preview_uses_runtime_values
        PASS [   0.061s] codex-tui bottom_pane::status_line_setup::tests::project_name_is_canonical_and_accepts_legacy_ids
        PASS [   0.086s] codex-tui bottom_pane::status_line_setup::tests::preview_uses_placeholders_when_runtime_values_are_missing
        PASS [   0.052s] codex-tui bottom_pane::status_line_setup::tests::run_state_is_canonical_and_accepts_status_legacy_id
        PASS [   0.075s] codex-tui bottom_pane::status_line_setup::tests::reasoning_is_selectable_id
        PASS [   0.045s] codex-tui bottom_pane::status_line_style::tests::pull_request_number_uses_link_style
        PASS [   0.067s] codex-tui bottom_pane::status_line_style::tests::status_line_segments_can_disable_theme_colors
        PASS [   0.068s] codex-tui bottom_pane::status_line_style::tests::status_line_segments_preserve_order_and_plain_text
        PASS [   0.075s] codex-tui bottom_pane::status_line_style::tests::status_line_segments_dim_separators_and_use_theme_styles_first
        PASS [   0.075s] codex-tui bottom_pane::status_line_style::tests::status_line_segments_return_none_when_empty
        PASS [   0.053s] codex-tui bottom_pane::status_line_style::tests::status_line_segments_soften_rgb_theme_styles_without_dimming_text
        PASS [   0.075s] codex-tui bottom_pane::tests::completing_top_view_preserves_underlying_view
        PASS [   0.731s] codex-tui bottom_pane::request_user_input::tests::request_user_input_unanswered_confirmation_snapshot
        PASS [   0.748s] codex-tui bottom_pane::request_user_input::tests::request_user_input_tight_height_snapshot
        PASS [   0.780s] codex-tui bottom_pane::request_user_input::tests::request_user_input_wrapped_options_snapshot
        PASS [   0.065s] codex-tui bottom_pane::tests::ctrl_c_cancels_history_search_without_clearing_draft_or_showing_quit_hint
        PASS [   0.249s] codex-tui bottom_pane::tests::approval_request_shows_immediately_without_recent_typing
        PASS [   0.107s] codex-tui bottom_pane::tests::dismiss_app_server_request_prunes_delayed_approval
        PASS [   0.269s] codex-tui bottom_pane::tests::approval_request_is_delayed_after_recent_typing
        PASS [   0.108s] codex-tui bottom_pane::tests::dismiss_app_server_request_removes_matching_buried_view
        PASS [   0.747s] codex-tui bottom_pane::skill_popup::tests::scrolling_mentions_shifts_rendered_window_snapshot
        PASS [   0.063s] codex-tui bottom_pane::tests::esc_interrupts_running_task_when_no_popup
        PASS [   0.320s] codex-tui bottom_pane::tests::continued_typing_resets_delayed_approval_idle_deadline
        PASS [   0.077s] codex-tui bottom_pane::tests::drain_pending_submission_state_clears_remote_image_urls
        PASS [   0.077s] codex-tui bottom_pane::tests::dismiss_app_server_request_returns_false_when_no_view_matches
        PASS [   0.763s] codex-tui bottom_pane::skills_toggle_view::tests::renders_basic_popup
        PASS [   0.090s] codex-tui bottom_pane::tests::esc_release_after_dismissing_agent_picker_does_not_interrupt_task
        PASS [   0.257s] codex-tui bottom_pane::tests::ctrl_c_on_modal_consumes_without_showing_quit_hint
        PASS [   0.247s] codex-tui bottom_pane::tests::delayed_approval_shortcut_works_after_idle_deadline
        PASS [   0.058s] codex-tui bottom_pane::tests::esc_routes_to_handle_key_event_when_requested
        PASS [   0.052s] codex-tui bottom_pane::tests::esc_with_skill_popup_does_not_interrupt_task
        PASS [   0.053s] codex-tui bottom_pane::tests::esc_with_agent_command_without_popup_does_not_interrupt_task
        PASS [   0.053s] codex-tui bottom_pane::tests::esc_with_slash_command_popup_does_not_interrupt_task
        PASS [   0.062s] codex-tui bottom_pane::tests::paste_completion_clears_stacked_views_and_restores_composer_input
        PASS [   0.069s] codex-tui bottom_pane::tests::release_events_are_ignored_for_active_view
        PASS [   0.074s] codex-tui bottom_pane::tests::remapped_interrupt_turn_uses_configured_key_including_agent_drafts
        PASS [   0.072s] codex-tui bottom_pane::tests::remote_images_render_above_composer_text
        PASS [   0.446s] codex-tui bottom_pane::tests::composer_shown_after_denied_while_task_running
        PASS [   0.070s] codex-tui bottom_pane::tests::selection_view_esc_respects_remapped_list_cancel
        PASS [   0.076s] codex-tui bottom_pane::tests::status_indicator_visible_during_command_execution
        PASS [   0.058s] codex-tui bottom_pane::tests::typed_approval_shortcuts_during_delay_stay_in_composer
        PASS [   0.065s] codex-tui bottom_pane::tests::unified_exec_summary_does_not_increase_height_when_status_visible
        PASS [   0.071s] codex-tui bottom_pane::textarea::tests::altgr_ctrl_alt_char_inserts_literal
        PASS [   0.059s] codex-tui bottom_pane::textarea::tests::c0_control_chars_respect_unbound_editor_movement
        PASS [   0.079s] codex-tui bottom_pane::textarea::tests::c0_control_chars_respect_remapped_editor_movement
        PASS [   0.054s] codex-tui bottom_pane::textarea::tests::control_b_and_f_move_cursor
        PASS [   0.071s] codex-tui bottom_pane::textarea::tests::c0_line_feed_inserts_newline_through_insert_newline_keymap
        PASS [   0.258s] codex-tui bottom_pane::tests::overlay_not_shown_above_approval_modal
        PASS [   0.057s] codex-tui bottom_pane::textarea::tests::control_b_f_fallback_control_chars_move_cursor
        PASS [   0.056s] codex-tui bottom_pane::textarea::tests::control_backspace_variants_delete_backward_word
        PASS [   0.748s] codex-tui bottom_pane::status_line_setup::tests::setup_view_snapshot_uses_runtime_preview_values
        PASS [   0.057s] codex-tui bottom_pane::textarea::tests::control_delete_variants_delete_forward_word
        PASS [   0.052s] codex-tui bottom_pane::textarea::tests::control_h_backspace
        PASS [   0.051s] codex-tui bottom_pane::textarea::tests::cursor_left_and_right_handle_graphemes
        PASS [   0.050s] codex-tui bottom_pane::textarea::tests::cursor_pos_with_state_after_movements
        PASS [   0.052s] codex-tui bottom_pane::textarea::tests::cursor_pos_with_state_basic_and_scroll_behaviors
        PASS [   0.066s] codex-tui bottom_pane::textarea::tests::cursor_vertical_movement_across_lines_and_bounds
        PASS [   0.059s] codex-tui bottom_pane::textarea::tests::delete_backward_and_forward_edges
        PASS [   0.085s] codex-tui bottom_pane::textarea::tests::delete_backward_word_alt_keys
        PASS [   0.074s] codex-tui bottom_pane::textarea::tests::delete_backward_word_and_kill_line_variants
        PASS [   0.059s] codex-tui bottom_pane::textarea::tests::delete_backward_word_handles_narrow_no_break_space
        PASS [   0.055s] codex-tui bottom_pane::textarea::tests::delete_backward_word_respects_word_separators
        PASS [   0.051s] codex-tui bottom_pane::textarea::tests::delete_forward_word_alt_d
        PASS [   0.063s] codex-tui bottom_pane::textarea::tests::delete_forward_deletes_element_at_left_edge
        PASS [   0.045s] codex-tui bottom_pane::textarea::tests::delete_forward_word_handles_atomic_elements
        PASS [   0.055s] codex-tui bottom_pane::textarea::tests::delete_forward_word_respects_word_separators
        PASS [   0.066s] codex-tui bottom_pane::textarea::tests::delete_forward_word_variants
        PASS [   0.072s] codex-tui bottom_pane::textarea::tests::delete_forward_word_with_without_alt_modifier
        PASS [   0.055s] codex-tui bottom_pane::textarea::tests::end_of_line_or_down_at_end_of_text
        PASS [   0.050s] codex-tui bottom_pane::textarea::tests::home_end_and_emacs_style_home_end
        PASS [   0.064s] codex-tui bottom_pane::textarea::tests::insert_and_replace_update_cursor_and_text
        PASS [   0.065s] codex-tui bottom_pane::textarea::tests::insert_str_at_clamps_to_char_boundary
        PASS [   0.074s] codex-tui bottom_pane::textarea::tests::kill_buffer_persists_across_set_text
        PASS [   0.079s] codex-tui bottom_pane::textarea::tests::kill_current_line_keeps_previous_newline_for_final_line
        PASS [   0.051s] codex-tui bottom_pane::textarea::tests::kill_current_line_removes_current_line_linewise
        PASS [   0.056s] codex-tui bottom_pane::textarea::tests::kill_whole_line_keymap_dispatch_uses_linewise_kill
        PASS [   0.062s] codex-tui bottom_pane::textarea::tests::render_highlights_apply_style_without_mutating_text
        PASS [   0.052s] codex-tui bottom_pane::textarea::tests::set_text_clamps_cursor_to_char_boundary
        PASS [   0.055s] codex-tui bottom_pane::textarea::tests::shift_backspace_and_shift_delete_keep_grapheme_delete_behavior
        PASS [   0.060s] codex-tui bottom_pane::textarea::tests::vim_c_at_line_end_enters_insert_without_removing_newline
        PASS [   0.052s] codex-tui bottom_pane::textarea::tests::vim_change_inner_word_deletes_word_and_enters_insert
        PASS [   0.051s] codex-tui bottom_pane::textarea::tests::vim_delete_to_word_end_advances_from_existing_word_end
        PASS [   0.063s] codex-tui bottom_pane::textarea::tests::vim_d_at_line_end_does_not_remove_newline
        PASS [   0.074s] codex-tui bottom_pane::textarea::tests::vim_delete_word
        PASS [   0.074s] codex-tui bottom_pane::textarea::tests::vim_delimiter_text_objects_select_innermost_pair_and_aliases
        PASS [   0.056s] codex-tui bottom_pane::textarea::tests::vim_dollar_lands_on_line_end_character
        PASS [   0.699s] codex-tui bottom_pane::tests::queued_messages_visible_when_status_hidden_snapshot
        PASS [   0.061s] codex-tui bottom_pane::textarea::tests::vim_e_advances_across_atomic_element_word_ends
        PASS [   0.064s] codex-tui bottom_pane::textarea::tests::vim_e_lands_on_word_end_character
        PASS [   0.072s] codex-tui bottom_pane::textarea::tests::vim_e_from_word_end_can_land_on_trailing_space
        PASS [   0.061s] codex-tui bottom_pane::textarea::tests::vim_empty_inner_text_objects_are_valid_targets
        PASS [   0.067s] codex-tui bottom_pane::textarea::tests::vim_escape_from_insert_at_line_start_stays_on_line
        PASS [   0.060s] codex-tui bottom_pane::textarea::tests::vim_escape_from_insert_at_start_does_not_underflow
        PASS [   0.764s] codex-tui bottom_pane::tests::status_and_composer_fill_height_without_bottom_padding
        PASS [   0.059s] codex-tui bottom_pane::textarea::tests::vim_escape_respects_atomic_element_boundary
        PASS [   0.747s] codex-tui bottom_pane::tests::status_only_snapshot
        PASS [   0.060s] codex-tui bottom_pane::textarea::tests::vim_escape_moves_by_grapheme_boundary
        PASS [   0.783s] codex-tui bottom_pane::tests::status_and_queued_messages_snapshot
        PASS [   0.070s] codex-tui bottom_pane::textarea::tests::vim_insert_and_escape
        PASS [   0.073s] codex-tui bottom_pane::textarea::tests::vim_insert_key_enters_insert_mode
        PASS [   0.772s] codex-tui bottom_pane::tests::status_with_details_and_queued_messages_snapshot
        PASS [   0.081s] codex-tui bottom_pane::textarea::tests::vim_linewise_yank_pastes_below_current_line
        PASS [   0.061s] codex-tui bottom_pane::textarea::tests::vim_normal_arrow_keys_move_cursor
        PASS [   0.062s] codex-tui bottom_pane::textarea::tests::vim_o_opens_line_below_final_line_and_moves_to_new_line
        PASS [   0.064s] codex-tui bottom_pane::textarea::tests::vim_o_opens_line_below_on_inserted_line
        PASS [   0.054s] codex-tui bottom_pane::textarea::tests::vim_s_on_empty_line_enters_insert_without_deleting_newline
        PASS [   0.067s] codex-tui bottom_pane::textarea::tests::vim_operator_invalid_motion_is_consumed
        PASS [   0.074s] codex-tui bottom_pane::textarea::tests::vim_quote_text_objects_are_line_local_and_handle_escapes
        PASS [   0.093s] codex-tui bottom_pane::textarea::tests::vim_s_substitutes_current_character_and_enters_insert_mode
        PASS [   0.085s] codex-tui bottom_pane::textarea::tests::vim_shift_a_enters_insert_at_line_end_with_shift_only_binding
        PASS [   0.086s] codex-tui bottom_pane::textarea::tests::vim_shift_c_changes_to_line_end_and_enters_insert_mode
        PASS [   0.093s] codex-tui bottom_pane::textarea::tests::vim_shift_i_enters_insert_at_first_non_blank_with_shift_only_binding
        PASS [   0.070s] codex-tui bottom_pane::textarea::tests::vim_text_object_cancellation_and_unsupported_change_motions_do_not_edit
        PASS [   0.078s] codex-tui bottom_pane::textarea::tests::vim_uppercase_c_changes_to_line_end
        PASS [   0.067s] codex-tui bottom_pane::textarea::tests::vim_word_text_objects_accept_cursor_at_word_end
        PASS [   0.090s] codex-tui bottom_pane::textarea::tests::vim_shift_o_opens_line_above_with_shift_only_binding
        PASS [   0.068s] codex-tui bottom_pane::textarea::tests::vim_word_text_objects_cover_delete_yank_and_big_word
        PASS [   0.038s] codex-tui bottom_pane::textarea::tests::word_navigation_cjk_each_char_is_boundary
        PASS [   0.049s] codex-tui bottom_pane::textarea::tests::word_navigation_cjk_forward
        PASS [   0.057s] codex-tui bottom_pane::textarea::tests::word_navigation_preserves_separator_breaks_within_unicode_segments
        PASS [   0.066s] codex-tui bottom_pane::textarea::tests::word_navigation_helpers
        PASS [   0.075s] codex-tui bottom_pane::textarea::tests::word_navigation_mixed_ascii_cjk
        PASS [   0.076s] codex-tui bottom_pane::textarea::tests::wrapped_navigation_across_visual_lines
        PASS [   0.072s] codex-tui bottom_pane::textarea::tests::wrapped_navigation_with_newlines_and_spaces
        PASS [   0.081s] codex-tui bottom_pane::textarea::tests::wrapped_navigation_with_wide_graphemes
        PASS [   0.090s] codex-tui bottom_pane::textarea::tests::wrapping_and_cursor_positions
        PASS [   0.079s] codex-tui bottom_pane::textarea::tests::yank_restores_last_kill
        PASS [   0.069s] codex-tui bottom_pane::title_setup::tests::activity_is_canonical_and_accepts_spinner_legacy_id
        PASS [   0.043s] codex-tui bottom_pane::title_setup::tests::parse_terminal_title_items_preserves_order
        PASS [   0.075s] codex-tui bottom_pane::title_setup::tests::model_with_reasoning_has_distinct_id
        PASS [   0.076s] codex-tui bottom_pane::title_setup::tests::parse_terminal_title_items_accepts_kebab_case_variants
        PASS [   0.101s] codex-tui bottom_pane::title_setup::tests::model_is_canonical_and_accepts_model_name_legacy_id
        PASS [   0.070s] codex-tui bottom_pane::title_setup::tests::parse_terminal_title_items_rejects_invalid_ids
        PASS [   0.072s] codex-tui bottom_pane::title_setup::tests::project_name_is_canonical_and_accepts_project_legacy_id
        PASS [   0.065s] codex-tui bottom_pane::title_setup::tests::reasoning_is_selectable_id
        PASS [   0.073s] codex-tui bottom_pane::title_setup::tests::run_state_is_canonical_and_accepts_status_legacy_id
        PASS [   0.072s] codex-tui bottom_pane::title_setup::tests::thread_title_is_canonical_and_accepts_thread_legacy_id
        PASS [   0.070s] codex-tui bottom_pane::unified_exec_footer::tests::desired_height_empty
        PASS [   0.078s] codex-tui branch_summary::tests::branch_diff_stats_prefers_remote_default_ref_over_stale_local_branch
        PASS [   0.085s] codex-tui branch_summary::tests::open_pull_request_falls_back_to_parent_repo_commit_lookup
        PASS [   0.085s] codex-tui branch_summary::tests::open_pull_request_uses_current_branch_view_first
        PASS [   0.079s] codex-tui branch_summary::tests::status_line_pr_fallback_searches_parent_repo_first
        PASS [   0.077s] codex-tui branch_summary::tests::status_line_pr_view_parser_requires_open_pr
        PASS [   0.084s] codex-tui chatwidget::goal_status::tests::active_goal_status_does_not_count_idle_time_before_turn_start
        PASS [   0.049s] codex-tui chatwidget::goal_status::tests::active_goal_usage_prefers_token_budget
        PASS [   0.078s] codex-tui chatwidget::goal_status::tests::active_goal_status_includes_current_turn_elapsed_time
        PASS [   0.060s] codex-tui chatwidget::goal_status::tests::completed_goal_usage_reports_time_without_token_budget
        PASS [   0.067s] codex-tui chatwidget::goal_status::tests::active_goal_usage_reports_time_without_budget
        PASS [   0.068s] codex-tui chatwidget::goal_status::tests::completed_goal_usage_reports_tokens_when_budgeted
        PASS [   0.051s] codex-tui chatwidget::goal_status::tests::stopped_goal_budget_usage_reports_budgeted_tokens
        PASS [   0.068s] codex-tui chatwidget::goal_status::tests::stopped_goal_budget_usage_omits_unbudgeted_usage
        PASS [   0.071s] codex-tui chatwidget::input_queue::tests::clear_resets_all_input_queues
        PASS [   0.071s] codex-tui chatwidget::input_queue::tests::preview_keeps_queue_categories_separate
        PASS [   0.078s] codex-tui chatwidget::interrupts::tests::remove_resolved_prompt_keeps_lifecycle_events
        PASS [   0.717s] codex-tui bottom_pane::textarea::tests::vim_e_advances_from_each_word_end
        PASS [   0.063s] codex-tui chatwidget::interrupts::tests::remove_resolved_prompt_matches_exec_approval_id
        PASS [   0.071s] codex-tui chatwidget::interrupts::tests::remove_resolved_prompt_removes_matching_user_input_only
        PASS [   0.053s] codex-tui chatwidget::reasoning_shortcuts::tests::next_reasoning_effort_clamps_at_bounds
        PASS [   0.049s] codex-tui chatwidget::reasoning_shortcuts::tests::next_reasoning_effort_lowers_from_default_anchor
        PASS [   0.072s] codex-tui chatwidget::reasoning_shortcuts::tests::next_reasoning_effort_does_not_infer_position_for_unsupported_current
        PASS [   0.059s] codex-tui chatwidget::reasoning_shortcuts::tests::next_reasoning_effort_raises_from_default_anchor
        PASS [   0.060s] codex-tui chatwidget::reasoning_shortcuts::tests::next_reasoning_effort_uses_advertised_order_for_custom_levels
        PASS [   0.072s] codex-tui chatwidget::reasoning_shortcuts::tests::next_reasoning_effort_single_option_is_noop
        PASS [   0.062s] codex-tui chatwidget::skills::tests::find_app_mentions_requires_accessible_enabled_apps_for_bound_paths
        PASS [   0.085s] codex-tui chatwidget::skills::tests::find_app_mentions_requires_accessible_enabled_apps_for_slugs
        PASS [   0.070s] codex-tui chatwidget::status_state::tests::guardian_status_aggregates_parallel_reviews
        PASS [   0.064s] codex-tui chatwidget::status_state::tests::retry_status_header_is_taken_once
        PASS [   0.121s] codex-tui chatwidget::tests::app_server::invalid_url_elicitation_is_declined
        PASS [   0.125s] codex-tui chatwidget::tests::app_server::collab_spawn_end_shows_requested_model_and_effort
        PASS [   0.137s] codex-tui chatwidget::tests::app_server::live_app_server_config_warning_prefixes_summary
        PASS [   0.137s] codex-tui chatwidget::tests::app_server::live_app_server_cyber_policy_error_renders_dedicated_notice
        PASS [   0.121s] codex-tui chatwidget::tests::app_server::live_app_server_failed_turn_does_not_duplicate_error_history
        PASS [   0.116s] codex-tui chatwidget::tests::app_server::live_app_server_guardian_warning_notification_renders_message
        PASS [   0.094s] codex-tui chatwidget::tests::app_server::live_app_server_invalid_thread_name_update_is_ignored
        PASS [   0.284s] codex-tui chatwidget::tests::app_server::live_app_server_failed_turn_consolidates_streamed_answer
        PASS [   0.103s] codex-tui chatwidget::tests::app_server::live_app_server_model_verification_renders_warning
        PASS [   0.772s] codex-tui bottom_pane::title_setup::tests::renders_title_setup_popup
        PASS [   0.744s] codex-tui bottom_pane::unified_exec_footer::tests::render_many_sessions
        PASS [   0.760s] codex-tui bottom_pane::unified_exec_footer::tests::render_more_sessions
        PASS [   0.255s] codex-tui chatwidget::tests::app_server::live_app_server_file_change_item_started_preserves_changes
        PASS [   0.121s] codex-tui chatwidget::tests::app_server::live_app_server_server_overloaded_error_renders_warning
        PASS [   0.126s] codex-tui chatwidget::tests::app_server::live_app_server_stream_recovery_restores_previous_status_header
        PASS [   0.145s] codex-tui chatwidget::tests::app_server::live_app_server_turn_completed_clears_working_status_after_answer_item
        PASS [   0.157s] codex-tui chatwidget::tests::app_server::live_app_server_thread_closed_requests_immediate_exit
        PASS [   0.118s] codex-tui chatwidget::tests::app_server::live_app_server_user_message_item_completed_does_not_duplicate_rendered_prompt
        PASS [   0.121s] codex-tui chatwidget::tests::app_server::live_app_server_turn_started_sets_feedback_turn_id
        PASS [   0.120s] codex-tui chatwidget::tests::app_server::live_app_server_warning_notification_renders_message
        PASS [   0.058s] codex-tui chatwidget::tests::approval_requests::app_server_exec_approval_request_preserves_permissions_context
        PASS [   0.070s] codex-tui chatwidget::tests::approval_requests::app_server_exec_approval_request_splits_shell_wrapped_command
        PASS [   0.117s] codex-tui chatwidget::tests::app_server::thread_settings_updated_preserves_default_settings_for_plan_mode
        PASS [   0.086s] codex-tui chatwidget::tests::approval_requests::app_server_request_permissions_preserves_file_system_permissions
        PASS [   0.113s] codex-tui chatwidget::tests::app_server::thread_settings_updated_updates_visible_state_without_transcript
        PASS [   0.791s] codex-tui chatwidget::tests::app_server::live_app_server_collab_wait_items_render_history
        PASS [   1.850s] codex-tui bottom_pane::textarea::tests::fuzz_textarea_randomized
        PASS [   0.815s] codex-tui chatwidget::tests::app_server::live_app_server_collab_spawn_completed_renders_requested_model_and_effort
        PASS [   0.102s] codex-tui chatwidget::tests::composer_submission::alt_up_edits_most_recent_queued_message
        PASS [   0.285s] codex-tui chatwidget::tests::approval_requests::exec_approval_uses_approval_id_when_present
        PASS [   0.111s] codex-tui chatwidget::tests::composer_submission::blocked_image_restore_with_remote_images_keeps_local_placeholder_mapping
        PASS [   0.920s] codex-tui chatwidget::tests::app_server::live_app_server_command_execution_strips_shell_wrapper
        PASS [   0.129s] codex-tui chatwidget::tests::composer_submission::blocked_image_restore_preserves_mention_bindings
        PASS [   0.102s] codex-tui chatwidget::tests::composer_submission::committed_user_message_with_hidden_prompt_context_renders_local_images
        PASS [   0.098s] codex-tui chatwidget::tests::composer_submission::empty_enter_during_task_does_not_queue
        PASS [   0.099s] codex-tui chatwidget::tests::composer_submission::enqueueing_history_prompt_multiple_times_is_stable
        PASS [   0.100s] codex-tui chatwidget::tests::composer_submission::enter_with_only_remote_images_does_not_submit_when_input_disabled
        PASS [   0.100s] codex-tui chatwidget::tests::composer_submission::enter_with_only_remote_images_does_not_submit_when_modal_is_active
        PASS [   0.090s] codex-tui chatwidget::tests::composer_submission::interrupt_prepends_queued_messages_before_existing_composer_text
        PASS [   0.122s] codex-tui chatwidget::tests::composer_submission::enter_with_only_remote_images_submits_user_turn
        PASS [   0.106s] codex-tui chatwidget::tests::composer_submission::interrupted_turn_restore_keeps_active_mode_for_resubmission
        PASS [   0.115s] codex-tui chatwidget::tests::composer_submission::interrupt_restores_queued_messages_into_composer
        PASS [   0.120s] codex-tui chatwidget::tests::composer_submission::output_free_interrupted_turn_requests_prompt_restore
        PASS [   0.778s] codex-tui chatwidget::tests::app_server::live_app_server_thread_name_update_shows_resume_hint
        PASS [   0.101s] codex-tui chatwidget::tests::composer_submission::patch_activity_prevents_cancelled_turn_prompt_restore
        PASS [   0.061s] codex-tui chatwidget::tests::composer_submission::queued_message_edit_binding_mapping_covers_special_terminals_and_tmux
        PASS [   0.114s] codex-tui chatwidget::tests::composer_submission::pending_steer_esc_does_not_steal_vim_insert_escape
        PASS [   0.071s] codex-tui chatwidget::tests::composer_submission::remap_placeholders_uses_attachment_labels
        PASS [   0.057s] codex-tui chatwidget::tests::composer_submission::remap_placeholders_uses_byte_ranges_when_placeholder_missing
        PASS [   0.107s] codex-tui chatwidget::tests::composer_submission::pending_steer_interrupt_uses_remapped_binding
        PASS [   0.115s] codex-tui chatwidget::tests::composer_submission::queued_restore_with_remote_images_keeps_local_placeholder_mapping
        PASS [  10.746s] codex-tui bottom_pane::chat_composer::tests::humanlike_typing_1000_chars_appears_live_no_placeholder
        PASS [   0.114s] codex-tui chatwidget::tests::composer_submission::restore_thread_input_state_syncs_sleep_inhibitor_state
        PASS [   0.115s] codex-tui chatwidget::tests::composer_submission::shift_enter_with_only_remote_images_does_not_submit_user_turn
        PASS [   0.140s] codex-tui chatwidget::tests::composer_submission::shift_left_edits_most_recent_queued_message_in_apple_terminal
        PASS [   0.125s] codex-tui chatwidget::tests::composer_submission::shift_left_edits_most_recent_queued_message_in_vscode_terminal
        PASS [   0.749s] codex-tui chatwidget::tests::approval_requests::network_exec_approval_history_describes_one_time_host_allowance
        PASS [   0.142s] codex-tui chatwidget::tests::composer_submission::shift_left_edits_most_recent_queued_message_in_tmux
        PASS [   0.117s] codex-tui chatwidget::tests::composer_submission::shift_left_edits_most_recent_queued_message_in_warp_terminal
        PASS [   0.128s] codex-tui chatwidget::tests::composer_submission::submission_includes_configured_active_permission_profile
        PASS [   0.142s] codex-tui chatwidget::tests::composer_submission::submission_omits_active_permission_profile_for_legacy_snapshot
        PASS [   0.134s] codex-tui chatwidget::tests::composer_submission::submission_prefers_selected_duplicate_skill_path
        PASS [   0.133s] codex-tui chatwidget::tests::composer_submission::submit_user_message_ignores_inaccessible_app_mentions_from_bindings
        PASS [   0.139s] codex-tui chatwidget::tests::composer_submission::submission_preserves_text_elements_and_local_images
        PASS [   0.958s] codex-tui chatwidget::tests::approval_requests::exec_approval_decision_truncates_multiline_and_long_commands
        PASS [   0.132s] codex-tui chatwidget::tests::composer_submission::thinking_status_keeps_cancelled_turn_prompt_restore_eligible
        PASS [   0.145s] codex-tui chatwidget::tests::composer_submission::submission_with_remote_and_local_images_keeps_local_placeholder_numbering
        PASS [   0.129s] codex-tui chatwidget::tests::composer_submission::unbound_queued_message_edit_does_not_fall_back_to_alt_up
        PASS [   0.055s] codex-tui chatwidget::tests::composer_submission::user_message_display_from_inputs_hides_prompt_context
        PASS [   0.059s] codex-tui chatwidget::tests::composer_submission::user_message_display_from_inputs_matches_flattened_user_message_shape
        PASS [   0.919s] codex-tui chatwidget::tests::approval_requests::network_exec_approval_history_describes_canceled_host_request
        PASS [   0.898s] codex-tui chatwidget::tests::approval_requests::network_exec_approval_history_describes_session_host_allowance
        PASS [   0.960s] codex-tui chatwidget::tests::approval_requests::exec_approval_emits_proposed_command_and_decision_history
        PASS [   0.045s] codex-tui chatwidget::tests::exec_flow::app_server_exec_approval_request_splits_shell_wrapped_command
        PASS [   0.107s] codex-tui chatwidget::tests::composer_submission::visible_output_prevents_cancelled_turn_prompt_restore
        PASS [   0.098s] codex-tui chatwidget::tests::exec_flow::apply_patch_approval_sends_op_with_call_id
        PASS [   0.113s] codex-tui chatwidget::tests::exec_flow::apply_patch_full_flow_integration_like
        PASS [   0.153s] codex-tui chatwidget::tests::exec_flow::apply_patch_request_omits_diff_summary_from_modal
        PASS [   0.164s] codex-tui chatwidget::tests::exec_flow::apply_patch_untrusted_shows_approval_modal
        PASS [   0.116s] codex-tui chatwidget::tests::exec_flow::bang_shell_enter_while_task_running_submits_run_user_shell_command
        PASS [   0.274s] codex-tui chatwidget::tests::exec_flow::apply_patch_events_emit_history_cells
        PASS [   0.305s] codex-tui chatwidget::tests::exec_flow::apply_patch_manual_approval_adjusts_header
        PASS [   0.285s] codex-tui chatwidget::tests::exec_flow::exec_approval_uses_approval_id_when_present
        PASS [   0.279s] codex-tui chatwidget::tests::exec_flow::exec_end_without_begin_does_not_flush_unrelated_running_exploring_cell
        PASS [   0.285s] codex-tui chatwidget::tests::exec_flow::exec_end_without_begin_flushes_completed_unrelated_exploring_cell
        PASS [   0.250s] codex-tui chatwidget::tests::exec_flow::exec_end_without_begin_uses_event_command
        PASS [   0.771s] codex-tui chatwidget::tests::config_errors::chained_config_error_wraps_in_history_snapshot
        PASS [   0.261s] codex-tui chatwidget::tests::exec_flow::exec_history_cell_shows_working_then_completed
        PASS [   0.764s] codex-tui chatwidget::tests::exec_flow::disabled_slash_command_while_task_running_snapshot
        PASS [   0.321s] codex-tui chatwidget::tests::exec_flow::exec_history_cell_shows_working_then_failed
        PASS [   0.122s] codex-tui chatwidget::tests::exec_flow::exec_history_shows_unified_exec_tool_calls
        PASS [   0.908s] codex-tui chatwidget::tests::exec_flow::approval_modal_patch_snapshot
        PASS [   0.962s] codex-tui chatwidget::tests::exec_flow::apply_patch_manual_flow_snapshot
        PASS [   0.112s] codex-tui chatwidget::tests::exec_flow::overlapping_exploring_exec_end_is_not_misclassified_as_orphan
        PASS [   1.060s] codex-tui chatwidget::tests::exec_flow::approval_modal_exec_multiline_prefix_hides_execpolicy_option_snapshot
        PASS [   0.129s] codex-tui chatwidget::tests::exec_flow::interrupt_preserves_unified_exec_processes
        PASS [   1.063s] codex-tui chatwidget::tests::exec_flow::approval_modal_exec_snapshot
        PASS [   0.341s] codex-tui chatwidget::tests::exec_flow::exec_history_shows_unified_exec_startup_commands
        PASS [   1.089s] codex-tui chatwidget::tests::exec_flow::approval_modal_exec_without_reason_snapshot
        PASS [   1.007s] codex-tui chatwidget::tests::exec_flow::exec_approval_decision_truncates_multiline_and_long_commands
        PASS [   0.108s] codex-tui chatwidget::tests::exec_flow::turn_complete_keeps_unified_exec_processes
        PASS [   1.018s] codex-tui chatwidget::tests::exec_flow::exec_approval_emits_proposed_command_and_decision_history
        PASS [   0.101s] codex-tui chatwidget::tests::exec_flow::unified_exec_empty_poll_for_finished_process_does_not_show_waiting_status
        PASS [   0.133s] codex-tui chatwidget::tests::exec_flow::unified_exec_begin_restores_status_indicator_after_preamble
        PASS [   0.098s] codex-tui chatwidget::tests::exec_flow::unified_exec_end_after_task_complete_is_suppressed
        PASS [   0.113s] codex-tui chatwidget::tests::exec_flow::unified_exec_interaction_after_task_complete_is_suppressed
        PASS [   0.114s] codex-tui chatwidget::tests::exec_flow::unified_exec_wait_status_header_updates_on_late_command_display
        PASS [   0.784s] codex-tui chatwidget::tests::exec_flow::exec_history_extends_previous_when_consecutive
        PASS [   0.796s] codex-tui chatwidget::tests::exec_flow::interrupt_preserves_unified_exec_wait_streak_snapshot
        PASS [   0.823s] codex-tui chatwidget::tests::exec_flow::image_generation_call_adds_history_cell
        PASS [   0.995s] codex-tui chatwidget::tests::exec_flow::final_worked_for_uses_cumulative_turn_duration_snapshot
        PASS [   0.842s] codex-tui chatwidget::tests::exec_flow::preamble_keeps_working_status_snapshot
        PASS [   0.159s] codex-tui chatwidget::tests::exec_flow::user_message_during_user_shell_command_is_queued_not_steered
        PASS [   0.872s] codex-tui chatwidget::tests::exec_flow::unified_exec_begin_restores_working_status_snapshot
        PASS [   0.763s] codex-tui chatwidget::tests::exec_flow::unified_exec_non_empty_then_empty_snapshots
        PASS [   0.835s] codex-tui chatwidget::tests::exec_flow::unified_exec_empty_then_non_empty_snapshot
        PASS [   0.139s] codex-tui chatwidget::tests::goal_menu::goal_edit_prompt_preserves_resumable_stopped_statuses
        PASS [   0.144s] codex-tui chatwidget::tests::goal_menu::goal_edit_prompt_resets_terminal_status_to_active
        PASS [   0.824s] codex-tui chatwidget::tests::exec_flow::unified_exec_wait_before_streamed_agent_message_snapshot
        PASS [   0.906s] codex-tui chatwidget::tests::exec_flow::unified_exec_wait_after_final_agent_message_snapshot
        PASS [   0.128s] codex-tui chatwidget::tests::goal_menu::goal_edit_prompt_submits_preserved_status_and_budget
        PASS [   1.015s] codex-tui chatwidget::tests::exec_flow::unified_exec_unknown_end_with_active_exploring_cell_snapshot
        PASS [   0.124s] codex-tui chatwidget::tests::goal_menu::resume_paused_goal_prompt_can_leave_goal_paused
        PASS [   0.088s] codex-tui chatwidget::tests::goal_menu::resume_paused_goal_prompt_default_resumes_goal
        PASS [   0.873s] codex-tui chatwidget::tests::exec_flow::unified_exec_wait_status_renders_command_in_single_details_row_snapshot
        PASS [   0.094s] codex-tui chatwidget::tests::goal_validation::goal_slash_command_accepts_multiline_objective_after_blank_first_line
        PASS [   0.140s] codex-tui chatwidget::tests::goal_validation::goal_slash_command_accepts_objective_at_limit
        PASS [   0.906s] codex-tui chatwidget::tests::exec_flow::unified_exec_waiting_multiple_empty_snapshots
        PASS [   0.152s] codex-tui chatwidget::tests::goal_validation::goal_slash_command_giant_paste_uses_goal_specific_error
        PASS [   0.117s] codex-tui chatwidget::tests::goal_validation::goal_slash_command_rejects_large_paste_using_expanded_length
        PASS [   0.115s] codex-tui chatwidget::tests::goal_validation::goal_slash_command_rejects_oversized_objective
        PASS [   0.773s] codex-tui chatwidget::tests::exec_flow::view_image_tool_call_adds_history_cell
        PASS [   0.126s] codex-tui chatwidget::tests::goal_validation::queued_goal_slash_command_rejects_oversized_objective_and_drains_next_input
        PASS [   0.798s] codex-tui chatwidget::tests::goal_menu::goal_edit_prompt_snapshot
        PASS [   0.129s] codex-tui chatwidget::tests::guardian::app_server_guardian_review_started_sets_review_status
        PASS [   0.800s] codex-tui chatwidget::tests::goal_menu::goal_menu_active_snapshot
        PASS [   0.119s] codex-tui chatwidget::tests::guardian::approving_recent_denial_emits_structured_core_op_once
        PASS [   0.761s] codex-tui chatwidget::tests::goal_menu::goal_menu_blocked_snapshot
        PASS [   1.096s] codex-tui chatwidget::tests::exec_flow::user_shell_command_renders_output_not_exploring
        PASS [   0.817s] codex-tui chatwidget::tests::goal_menu::goal_menu_budget_limited_snapshot
        PASS [   0.809s] codex-tui chatwidget::tests::goal_menu::goal_menu_usage_limited_snapshot
        PASS [   0.879s] codex-tui chatwidget::tests::goal_menu::goal_menu_paused_snapshot
        PASS [   0.133s] codex-tui chatwidget::tests::guardian::guardian_parallel_reviews_keep_remaining_review_visible_after_denial
        PASS [   0.776s] codex-tui chatwidget::tests::goal_menu::resume_paused_goal_prompt_snapshot
        PASS [   0.853s] codex-tui chatwidget::tests::guardian::app_server_guardian_review_denied_renders_denied_request_snapshot
        PASS [   0.807s] codex-tui chatwidget::tests::guardian::app_server_guardian_review_timed_out_renders_timed_out_request_snapshot
        PASS [   0.858s] codex-tui chatwidget::tests::guardian::guardian_approved_exec_renders_approved_request
        PASS [   0.901s] codex-tui chatwidget::tests::guardian::auto_review_denials_popup_lists_stored_auto_review_denials
        PASS [   0.861s] codex-tui chatwidget::tests::guardian::guardian_approved_request_permissions_renders_request_summary
        PASS [   0.858s] codex-tui chatwidget::tests::guardian::guardian_denied_exec_renders_warning_and_denied_request
        PASS [   0.854s] codex-tui chatwidget::tests::guardian::guardian_timed_out_exec_renders_warning_and_timed_out_request
        PASS [   0.135s] codex-tui chatwidget::tests::history_replay::live_reasoning_summary_is_not_rendered_twice_when_item_completes
        PASS [   0.135s] codex-tui chatwidget::tests::history_replay::replayed_in_progress_mcp_tool_call_stays_active
        PASS [   0.922s] codex-tui chatwidget::tests::guardian::guardian_parallel_reviews_render_aggregate_status_snapshot
        PASS [   0.130s] codex-tui chatwidget::tests::history_replay::replayed_reasoning_item_shows_raw_reasoning_when_enabled
        PASS [   0.813s] codex-tui chatwidget::tests::history_replay::app_server_forked_thread_history_line_uses_app_server_title_snapshot
        PASS [   0.150s] codex-tui chatwidget::tests::history_replay::replayed_in_progress_turn_marks_task_running
        PASS [   0.145s] codex-tui chatwidget::tests::history_replay::replayed_reasoning_item_hides_raw_reasoning_when_disabled
        PASS [   0.833s] codex-tui chatwidget::tests::history_replay::forked_thread_history_line_includes_name_and_id_snapshot
        PASS [   0.118s] codex-tui chatwidget::tests::history_replay::replayed_retryable_app_server_error_keeps_turn_running
        PASS [   0.878s] codex-tui chatwidget::tests::history_replay::app_server_forked_thread_history_line_without_app_server_name_ignores_local_snapshot
        PASS [   0.101s] codex-tui chatwidget::tests::history_replay::replayed_review_prompt_does_not_seed_composer_history
        PASS [   0.826s] codex-tui chatwidget::tests::history_replay::forked_thread_history_line_without_name_shows_id_once_snapshot
        PASS [   0.099s] codex-tui chatwidget::tests::history_replay::replayed_stream_error_does_not_set_retry_status_or_status_indicator
        PASS [   0.116s] codex-tui chatwidget::tests::history_replay::replayed_thread_closed_notification_does_not_exit_tui
        PASS [   0.123s] codex-tui chatwidget::tests::history_replay::replayed_user_message_preserves_remote_image_urls
        PASS [   0.145s] codex-tui chatwidget::tests::history_replay::replayed_user_message_preserves_text_elements_and_local_images
        PASS [   0.169s] codex-tui chatwidget::tests::history_replay::replayed_user_message_with_only_local_images_renders_history_cell
        PASS [   0.112s] codex-tui chatwidget::tests::history_replay::session_configured_preserves_profile_workspace_roots
        PASS [   0.177s] codex-tui chatwidget::tests::history_replay::replayed_user_message_with_only_remote_images_renders_history_cell
        PASS [   0.160s] codex-tui chatwidget::tests::history_replay::replayed_user_messages_seed_composer_history
        PASS [   0.170s] codex-tui chatwidget::tests::history_replay::resumed_initial_messages_render_history
        PASS [   0.130s] codex-tui chatwidget::tests::history_replay::session_configured_syncs_widget_config_permissions_and_cwd
        PASS [   0.121s] codex-tui chatwidget::tests::history_replay::stream_recovery_restores_previous_status_header
        PASS [   0.163s] codex-tui chatwidget::tests::history_replay::session_configured_external_sandbox_keeps_external_runtime_policy
        PASS [   0.105s] codex-tui chatwidget::tests::history_replay::thread_snapshot_replayed_stream_recovery_restores_previous_status_header
        PASS [   0.112s] codex-tui chatwidget::tests::history_replay::thread_snapshot_replayed_turn_started_marks_task_running
        PASS [   0.138s] codex-tui chatwidget::tests::history_replay::thread_snapshot_replay_preserves_agent_message_during_review_mode
        PASS [   0.099s] codex-tui chatwidget::tests::mcp_startup::app_server_mcp_startup_after_lag_can_settle_without_starting_updates
        PASS [   0.094s] codex-tui chatwidget::tests::mcp_startup::app_server_mcp_startup_after_lag_includes_runtime_servers_with_expected_set
        PASS [   0.122s] codex-tui chatwidget::tests::mcp_startup::app_server_mcp_startup_after_lag_preserves_partial_terminal_only_round
        PASS [   0.119s] codex-tui chatwidget::tests::mcp_startup::app_server_mcp_startup_next_round_after_lag_can_settle_without_starting_updates
        PASS [   0.135s] codex-tui chatwidget::tests::mcp_startup::app_server_mcp_startup_lag_settles_startup_and_ignores_late_updates
        PASS [   0.137s] codex-tui chatwidget::tests::mcp_startup::app_server_mcp_startup_next_round_discards_stale_terminal_updates
        PASS [   0.136s] codex-tui chatwidget::tests::mcp_startup::app_server_mcp_startup_next_round_with_empty_expected_servers_reactivates
        PASS [   0.147s] codex-tui chatwidget::tests::mcp_startup::app_server_mcp_startup_next_round_keeps_terminal_statuses_after_starting
        PASS [   0.151s] codex-tui chatwidget::tests::mcp_startup::mcp_startup_complete_does_not_clear_running_task
        PASS [   0.144s] codex-tui chatwidget::tests::mcp_startup::mcp_startup_failure_restores_running_status_header
        PASS [   0.150s] codex-tui chatwidget::tests::mcp_startup::mcp_startup_complete_preserves_review_status
        PASS [   0.128s] codex-tui chatwidget::tests::mcp_startup::turn_start_preserves_active_mcp_startup_header
        PASS [   0.121s] codex-tui chatwidget::tests::mcp_startup::turn_start_replaces_idle_completed_mcp_startup_header
        PASS [   0.134s] codex-tui chatwidget::tests::permissions::approvals_popup_shows_disabled_presets
  TRY 1 FAIL [   0.122s] codex-tui chatwidget::tests::permissions::permissions_full_access_history_cell_emitted_only_after_confirmation
  stdout ---

    running 1 test
    test chatwidget::tests::permissions::permissions_full_access_history_cell_emitted_only_after_confirmation ... FAILED

    failures:

    failures:
        chatwidget::tests::permissions::permissions_full_access_history_cell_emitted_only_after_confirmation

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.06s

  stderr ---

    thread 'chatwidget::tests::permissions::permissions_full_access_history_cell_emitted_only_after_confirmation' (3584) panicked at tui\src\chatwidget\tests\permissions.rs:1158:33:
    expected full access confirmation event
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.124s] codex-tui chatwidget::tests::permissions::permissions_selection_can_disable_auto_review
        PASS [   0.111s] codex-tui chatwidget::tests::permissions::permissions_selection_emits_history_cell_when_current_is_selected
        PASS [   0.140s] codex-tui chatwidget::tests::permissions::permissions_selection_emits_history_cell_when_selection_changes
  TRY 1 FAIL [   0.234s] codex-tui chatwidget::tests::permissions::approvals_popup_navigation_skips_disabled
  stdout ---

    running 1 test
    test chatwidget::tests::permissions::approvals_popup_navigation_skips_disabled ... FAILED

    failures:

    failures:
        chatwidget::tests::permissions::approvals_popup_navigation_skips_disabled

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.16s

  stderr ---

    thread 'chatwidget::tests::permissions::approvals_popup_navigation_skips_disabled' (26360) panicked at tui\src\chatwidget\tests\permissions.rs:695:5:
    enter should select an enabled preset
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.132s] codex-tui chatwidget::tests::permissions::permissions_selection_hides_auto_review_when_feature_disabled
        PASS [   0.127s] codex-tui chatwidget::tests::permissions::permissions_selection_hides_auto_review_when_feature_disabled_even_if_auto_review_is_active
  TRY 2 FAIL [   0.111s] codex-tui chatwidget::tests::permissions::permissions_full_access_history_cell_emitted_only_after_confirmation
  stdout ---

    running 1 test
    test chatwidget::tests::permissions::permissions_full_access_history_cell_emitted_only_after_confirmation ... FAILED

    failures:

    failures:
        chatwidget::tests::permissions::permissions_full_access_history_cell_emitted_only_after_confirmation

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.05s

  stderr ---

    thread 'chatwidget::tests::permissions::permissions_full_access_history_cell_emitted_only_after_confirmation' (22688) panicked at tui\src\chatwidget\tests\permissions.rs:1158:33:
    expected full access confirmation event
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.139s] codex-tui chatwidget::tests::permissions::permissions_selection_marks_auto_review_current_after_session_configured
        PASS [   0.080s] codex-tui chatwidget::tests::permissions::preset_matching_accepts_workspace_write_with_extra_roots
        PASS [   0.159s] codex-tui chatwidget::tests::permissions::permissions_selection_marks_auto_review_current_with_custom_workspace_write_details
        PASS [   0.176s] codex-tui chatwidget::tests::permissions::permissions_selection_sends_approvals_reviewer_in_override_turn_context
        PASS [   0.077s] codex-tui chatwidget::tests::permissions::preset_matching_does_not_treat_non_cwd_writable_profile_as_read_only
  TRY 2 FAIL [   0.229s] codex-tui chatwidget::tests::permissions::approvals_popup_navigation_skips_disabled
  stdout ---

    running 1 test
    test chatwidget::tests::permissions::approvals_popup_navigation_skips_disabled ... FAILED

    failures:

    failures:
        chatwidget::tests::permissions::approvals_popup_navigation_skips_disabled

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.17s

  stderr ---

    thread 'chatwidget::tests::permissions::approvals_popup_navigation_skips_disabled' (17440) panicked at tui\src\chatwidget\tests\permissions.rs:695:5:
    enter should select an enabled preset
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.467s] codex-tui chatwidget::tests::permissions::approvals_selection_popup_snapshot_windows_degraded_sandbox
        PASS [   0.125s] codex-tui chatwidget::tests::permissions::profile_permissions_selection_emits_active_custom_profile
  TRY 1 FAIL [   0.168s] codex-tui chatwidget::tests::permissions::profile_permissions_full_access_opens_confirmation
  stdout ---

    running 1 test
    test chatwidget::tests::permissions::profile_permissions_full_access_opens_confirmation ... FAILED

    failures:

    failures:
        chatwidget::tests::permissions::profile_permissions_full_access_opens_confirmation

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.06s

  stderr ---

    thread 'chatwidget::tests::permissions::profile_permissions_full_access_opens_confirmation' (19204) panicked at tui\src\chatwidget\tests\permissions.rs:247:5:
    assertion failed: matches!(&events[0], AppEvent::OpenFullAccessConfirmation
        {
            preset, return_to_permissions: true, profile_selection:
            Some(PermissionProfileSelection
            {
                profile_id, approval_policy: Some(AskForApproval::Never),
                approvals_reviewer: Some(ApprovalsReviewer::User), display_label,
            }),
        } if preset.id == "full-access" && profile_id == ":danger-no-sandbox" &&
        display_label == "Full Access")
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.847s] codex-tui chatwidget::tests::mcp_startup::app_server_mcp_startup_failure_renders_warning_history
        PASS [   0.273s] codex-tui chatwidget::tests::permissions::profile_permissions_selection_emits_auto_review_mode_event
        PASS [   0.174s] codex-tui chatwidget::tests::permissions::required_windows_sandbox_setup_defers_configured_initial_prompt
  TRY 2 FAIL [   0.191s] codex-tui chatwidget::tests::permissions::profile_permissions_full_access_opens_confirmation
  stdout ---

    running 1 test
    test chatwidget::tests::permissions::profile_permissions_full_access_opens_confirmation ... FAILED

    failures:

    failures:
        chatwidget::tests::permissions::profile_permissions_full_access_opens_confirmation

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.09s

  stderr ---

    thread 'chatwidget::tests::permissions::profile_permissions_full_access_opens_confirmation' (18256) panicked at tui\src\chatwidget\tests\permissions.rs:247:5:
    assertion failed: matches!(&events[0], AppEvent::OpenFullAccessConfirmation
        {
            preset, return_to_permissions: true, profile_selection:
            Some(PermissionProfileSelection
            {
                profile_id, approval_policy: Some(AskForApproval::Never),
                approvals_reviewer: Some(ApprovalsReviewer::User), display_label,
            }),
        } if preset.id == "full-access" && profile_id == ":danger-no-sandbox" &&
        display_label == "Full Access")
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.291s] codex-tui chatwidget::tests::permissions::profile_permissions_selection_emits_named_profile_event_only
        PASS [   0.866s] codex-tui chatwidget::tests::mcp_startup::mcp_startup_header_booting_snapshot
        PASS [   0.813s] codex-tui chatwidget::tests::permissions::full_access_confirmation_popup_snapshot
        PASS [   0.160s] codex-tui chatwidget::tests::permissions::startup_does_not_prompt_for_windows_sandbox_when_not_requested
        PASS [   0.183s] codex-tui chatwidget::tests::permissions::startup_windows_sandbox_prompt_blocks_disallowed_unelevated_fallback
        PASS [   0.187s] codex-tui chatwidget::tests::permissions::windows_sandbox_required_enable_prompt_reopens_on_cancel_when_unelevated_allowed
        PASS [   0.191s] codex-tui chatwidget::tests::permissions::windows_auto_mode_prompt_requests_enabling_sandbox_feature
        PASS [   0.194s] codex-tui chatwidget::tests::permissions::startup_prompts_for_windows_sandbox_when_agent_requested
  TRY 1 FAIL [   0.994s] codex-tui chatwidget::tests::permissions::approvals_selection_popup_snapshot
  stdout ---

    running 1 test
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Snapshot Summary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Snapshot file: tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__approvals_selection_popup@windows.snap
    Snapshot: codex_tui__chatwidget__tests__approvals_selection_popup@windows
    Source: C:\CodexLab\codex\codex-rs:81
    ───────────────────────────────────────────────────────────────────────────────
    Expression: popup
    ───────────────────────────────────────────────────────────────────────────────
    -old snapshot
    +new results
    ────────────┬──────────────────────────────────────────────────────────────────
        1     1 │   Update Model Permissions
        2     2 │
        3       │-› 1. Read Only (current)  Codex can read files in the current workspace.
        4       │-                          Approval is required to edit files or access the
        5       │-                          internet.
        6       │-  2. Ask for approval     Codex can read and edit files in the current
        7       │-                          workspace, and run commands. Approval is required to
        8       │-                          access the internet or edit other files.
        9       │-  3. Full Access          Codex can edit files outside this workspace and
       10       │-                          access the internet without asking for approval.
       11       │-                          Exercise caution when using.
              3 │+› 1. Read Only (current)        Codex can read files in the current workspace.
              4 │+                                Approval is required to edit files or access
              5 │+                                the internet.
              6 │+  2. Ask for approval           Codex can read and edit files in the current
              7 │+                                workspace, and run commands. Approval is
              8 │+                                required to access the internet or edit other
              9 │+                                files.
             10 │+  3. Cool Read Write (current)  Codex can read and write files within the
             11 │+                                workspace and execute commands, but will never
             12 │+                                read or write files outside the workspace. No
             13 │+                                internet access. (Designed for safety)
             14 │+  4. Full Access                Codex can edit files outside this workspace
             15 │+                                and access the internet without asking for
             16 │+                                approval. Exercise caution when using.
       12    17 │
       13    18 │   Press enter to confirm or esc to go back
    ────────────┴──────────────────────────────────────────────────────────────────
    To update snapshots run `cargo insta review`
    Stopped on the first failure. Run `cargo insta test` to run all snapshots.
    test chatwidget::tests::permissions::approvals_selection_popup_snapshot ... FAILED

    failures:

    failures:
        chatwidget::tests::permissions::approvals_selection_popup_snapshot

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.93s

  stderr ---
    stored new snapshot C:\CodexLab\codex\codex-rs\tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__approvals_selection_popup@windows.snap.new

    thread 'chatwidget::tests::permissions::approvals_selection_popup_snapshot' (4352) panicked at C:\Users\JohnDell\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\insta-1.46.3\src\runtime.rs:719:13:
    snapshot assertion for 'codex_tui__chatwidget__tests__approvals_selection_popup@windows' failed in line 81
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.108s] codex-tui chatwidget::tests::plan_mode::agent_turn_complete_does_not_override_pending_plan_mode_prompt_notification
        PASS [   0.115s] codex-tui chatwidget::tests::plan_mode::collab_mode_is_sent_after_enabling
        PASS [   0.135s] codex-tui chatwidget::tests::plan_mode::collab_mode_applies_default_preset
        PASS [   0.135s] codex-tui chatwidget::tests::plan_mode::collaboration_modes_defaults_to_code_on_startup
  TRY 1 FAIL [   0.917s] codex-tui chatwidget::tests::permissions::permissions_selection_history_snapshot_after_mode_switch
  stdout ---

    running 1 test
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Snapshot Summary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Snapshot file: tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__permissions_selection_history_after_mode_switch.snap
    Snapshot: codex_tui__chatwidget__tests__permissions_selection_history_after_mode_switch
    Source: C:\CodexLab\codex\codex-rs:765
    ───────────────────────────────────────────────────────────────────────────────
    Expression: lines_to_single_string(&cells[0])
    ───────────────────────────────────────────────────────────────────────────────
    -old snapshot
    +new results
    ────────────┬──────────────────────────────────────────────────────────────────
        1       │-• Permissions updated to Full Access
              1 │+ⓘ Permissions updated to Cool Read Write
    ────────────┴──────────────────────────────────────────────────────────────────
    To update snapshots run `cargo insta review`
    Stopped on the first failure. Run `cargo insta test` to run all snapshots.
    test chatwidget::tests::permissions::permissions_selection_history_snapshot_after_mode_switch ... FAILED

    failures:

    failures:
        chatwidget::tests::permissions::permissions_selection_history_snapshot_after_mode_switch

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.85s

  stderr ---
    stored new snapshot C:\CodexLab\codex\codex-rs\tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__permissions_selection_history_after_mode_switch.snap.new

    thread 'chatwidget::tests::permissions::permissions_selection_history_snapshot_after_mode_switch' (12448) panicked at C:\Users\JohnDell\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\insta-1.46.3\src\runtime.rs:719:13:
    snapshot assertion for 'codex_tui__chatwidget__tests__permissions_selection_history_after_mode_switch' failed in line 765
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.147s] codex-tui chatwidget::tests::plan_mode::collab_mode_shift_tab_cycles_only_when_idle
        PASS [   0.122s] codex-tui chatwidget::tests::plan_mode::enter_submits_when_plan_stream_is_not_active
        PASS [   0.110s] codex-tui chatwidget::tests::plan_mode::handle_request_user_input_sets_pending_notification
        PASS [   0.106s] codex-tui chatwidget::tests::plan_mode::mode_switch_surfaces_reasoning_change_notification_when_model_stays_same
  TRY 1 FAIL [   1.012s] codex-tui chatwidget::tests::permissions::permissions_selection_history_snapshot_full_access_to_default
  stdout ---

    running 1 test
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Snapshot Summary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Snapshot file: tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__permissions_selection_history_full_access_to_default@windows.snap
    Snapshot: codex_tui__chatwidget__tests__permissions_selection_history_full_access_to_default@windows
    Source: C:\CodexLab\codex\codex-rs:802
    ───────────────────────────────────────────────────────────────────────────────
    Expression: lines_to_single_string(&cells[0])
    ───────────────────────────────────────────────────────────────────────────────
    -old snapshot
    +new results
    ────────────┬──────────────────────────────────────────────────────────────────
        1       │-• Permissions updated to Ask for approval (non-admin sandbox)
              1 │+ⓘ Permissions updated to Approve for me
    ────────────┴──────────────────────────────────────────────────────────────────
    To update snapshots run `cargo insta review`
    Stopped on the first failure. Run `cargo insta test` to run all snapshots.
    test chatwidget::tests::permissions::permissions_selection_history_snapshot_full_access_to_default ... FAILED

    failures:

    failures:
        chatwidget::tests::permissions::permissions_selection_history_snapshot_full_access_to_default

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.94s

  stderr ---
    stored new snapshot C:\CodexLab\codex\codex-rs\tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__permissions_selection_history_full_access_to_default@windows.snap.new

    thread 'chatwidget::tests::permissions::permissions_selection_history_snapshot_full_access_to_default' (25728) panicked at C:\Users\JohnDell\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\insta-1.46.3\src\runtime.rs:719:13:
    snapshot assertion for 'codex_tui__chatwidget__tests__permissions_selection_history_full_access_to_default@windows' failed in line 802
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.126s] codex-tui chatwidget::tests::plan_mode::mode_switch_surfaces_model_change_notification_when_effective_model_changes
        PASS [   0.124s] codex-tui chatwidget::tests::plan_mode::open_plan_implementation_prompt_sets_pending_notification
        PASS [   0.110s] codex-tui chatwidget::tests::plan_mode::open_plan_reasoning_scope_prompt_sets_pending_notification
        PASS [   0.112s] codex-tui chatwidget::tests::plan_mode::plan_implementation_popup_clear_context_emits_clear_submit_event
        PASS [   0.138s] codex-tui chatwidget::tests::plan_mode::plan_completion_restores_status_indicator_after_streaming_plan_output
        PASS [   0.124s] codex-tui chatwidget::tests::plan_mode::plan_implementation_clear_context_requires_default_mode_and_plan
        PASS [   0.838s] codex-tui chatwidget::tests::permissions::profile_permissions_selection_popup_with_custom_profiles_snapshot
        PASS [   0.893s] codex-tui chatwidget::tests::permissions::profile_permissions_selection_popup_snapshot
        PASS [   0.134s] codex-tui chatwidget::tests::plan_mode::plan_implementation_popup_shows_after_proposed_plan_output
        PASS [   0.140s] codex-tui chatwidget::tests::plan_mode::plan_implementation_popup_shows_after_new_plan_follows_steer
        PASS [   0.143s] codex-tui chatwidget::tests::plan_mode::plan_implementation_popup_skips_replayed_turn_complete
        PASS [   0.150s] codex-tui chatwidget::tests::plan_mode::plan_implementation_popup_shows_once_when_replay_precedes_live_turn_complete
        PASS [   0.138s] codex-tui chatwidget::tests::plan_mode::plan_implementation_popup_skips_when_messages_queued
        PASS [   0.130s] codex-tui chatwidget::tests::plan_mode::plan_implementation_popup_skips_when_steer_follows_proposed_plan
        PASS [   0.127s] codex-tui chatwidget::tests::plan_mode::plan_implementation_popup_skips_without_proposed_plan
        PASS [   0.100s] codex-tui chatwidget::tests::plan_mode::plan_implementation_popup_yes_emits_submit_message_event
        PASS [   0.809s] codex-tui chatwidget::tests::permissions::windows_sandbox_required_enable_prompt_snapshot
        PASS [   0.158s] codex-tui chatwidget::tests::plan_mode::plan_implementation_popup_skips_when_rate_limit_prompt_pending
        PASS [   0.794s] codex-tui chatwidget::tests::permissions::windows_sandbox_required_fallback_prompt_snapshot
        PASS [   0.059s] codex-tui chatwidget::tests::plan_mode::plan_mode_nudge_matches_only_standalone_plain_text_keyword
        PASS [   0.102s] codex-tui chatwidget::tests::plan_mode::plan_mode_nudge_dismissal_is_scoped_to_current_thread
        PASS [   0.144s] codex-tui chatwidget::tests::plan_mode::plan_mode_nudge_hides_while_task_or_modal_is_active
        PASS [   0.135s] codex-tui chatwidget::tests::plan_mode::plan_mode_nudge_shift_tab_uses_existing_mode_cycle_path
        PASS [   0.153s] codex-tui chatwidget::tests::plan_mode::plan_mode_nudge_shows_only_for_eligible_default_mode_drafts
        PASS [   0.077s] codex-tui chatwidget::tests::plan_mode::plan_mode_prompt_notification_uses_dedicated_type_name
        PASS [   0.165s] codex-tui chatwidget::tests::plan_mode::plan_mode_reasoning_override_is_marked_current_in_reasoning_popup
        PASS [   0.165s] codex-tui chatwidget::tests::plan_mode::plan_reasoning_scope_popup_all_modes_persists_global_and_plan_override
        PASS [   0.145s] codex-tui chatwidget::tests::plan_mode::plan_reasoning_scope_popup_mentions_built_in_plan_default_when_no_override
  TRY 2 FAIL [   0.943s] codex-tui chatwidget::tests::permissions::approvals_selection_popup_snapshot
  stdout ---

    running 1 test
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Snapshot Summary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Snapshot file: tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__approvals_selection_popup@windows.snap
    Snapshot: codex_tui__chatwidget__tests__approvals_selection_popup@windows
    Source: C:\CodexLab\codex\codex-rs:81
    ───────────────────────────────────────────────────────────────────────────────
    Expression: popup
    ───────────────────────────────────────────────────────────────────────────────
    -old snapshot
    +new results
    ────────────┬──────────────────────────────────────────────────────────────────
        1     1 │   Update Model Permissions
        2     2 │
        3       │-› 1. Read Only (current)  Codex can read files in the current workspace.
        4       │-                          Approval is required to edit files or access the
        5       │-                          internet.
        6       │-  2. Ask for approval     Codex can read and edit files in the current
        7       │-                          workspace, and run commands. Approval is required to
        8       │-                          access the internet or edit other files.
        9       │-  3. Full Access          Codex can edit files outside this workspace and
       10       │-                          access the internet without asking for approval.
       11       │-                          Exercise caution when using.
              3 │+› 1. Read Only (current)        Codex can read files in the current workspace.
              4 │+                                Approval is required to edit files or access
              5 │+                                the internet.
              6 │+  2. Ask for approval           Codex can read and edit files in the current
              7 │+                                workspace, and run commands. Approval is
              8 │+                                required to access the internet or edit other
              9 │+                                files.
             10 │+  3. Cool Read Write (current)  Codex can read and write files within the
             11 │+                                workspace and execute commands, but will never
             12 │+                                read or write files outside the workspace. No
             13 │+                                internet access. (Designed for safety)
             14 │+  4. Full Access                Codex can edit files outside this workspace
             15 │+                                and access the internet without asking for
             16 │+                                approval. Exercise caution when using.
       12    17 │
       13    18 │   Press enter to confirm or esc to go back
    ────────────┴──────────────────────────────────────────────────────────────────
    To update snapshots run `cargo insta review`
    Stopped on the first failure. Run `cargo insta test` to run all snapshots.
    test chatwidget::tests::permissions::approvals_selection_popup_snapshot ... FAILED

    failures:

    failures:
        chatwidget::tests::permissions::approvals_selection_popup_snapshot

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.87s

  stderr ---
    stored new snapshot C:\CodexLab\codex\codex-rs\tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__approvals_selection_popup@windows.snap.new

    thread 'chatwidget::tests::permissions::approvals_selection_popup_snapshot' (18756) panicked at C:\Users\JohnDell\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\insta-1.46.3\src\runtime.rs:719:13:
    snapshot assertion for 'codex_tui__chatwidget__tests__approvals_selection_popup@windows' failed in line 81
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.132s] codex-tui chatwidget::tests::plan_mode::plan_reasoning_scope_popup_mentions_selected_reasoning
  TRY 2 FAIL [   0.886s] codex-tui chatwidget::tests::permissions::permissions_selection_history_snapshot_after_mode_switch
  stdout ---

    running 1 test
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Snapshot Summary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Snapshot file: tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__permissions_selection_history_after_mode_switch.snap
    Snapshot: codex_tui__chatwidget__tests__permissions_selection_history_after_mode_switch
    Source: C:\CodexLab\codex\codex-rs:765
    ───────────────────────────────────────────────────────────────────────────────
    Expression: lines_to_single_string(&cells[0])
    ───────────────────────────────────────────────────────────────────────────────
    -old snapshot
    +new results
    ────────────┬──────────────────────────────────────────────────────────────────
        1       │-• Permissions updated to Full Access
              1 │+ⓘ Permissions updated to Cool Read Write
    ────────────┴──────────────────────────────────────────────────────────────────
    To update snapshots run `cargo insta review`
    Stopped on the first failure. Run `cargo insta test` to run all snapshots.
    test chatwidget::tests::permissions::permissions_selection_history_snapshot_after_mode_switch ... FAILED

    failures:

    failures:
        chatwidget::tests::permissions::permissions_selection_history_snapshot_after_mode_switch

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.82s

  stderr ---
    stored new snapshot C:\CodexLab\codex\codex-rs\tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__permissions_selection_history_after_mode_switch.snap.new

    thread 'chatwidget::tests::permissions::permissions_selection_history_snapshot_after_mode_switch' (23024) panicked at C:\Users\JohnDell\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\insta-1.46.3\src\runtime.rs:719:13:
    snapshot assertion for 'codex_tui__chatwidget__tests__permissions_selection_history_after_mode_switch' failed in line 765
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.110s] codex-tui chatwidget::tests::plan_mode::plan_reasoning_scope_popup_plan_only_does_not_update_all_modes_reasoning
        PASS [   0.121s] codex-tui chatwidget::tests::plan_mode::plan_slash_command_switches_to_plan_mode
        PASS [   0.136s] codex-tui chatwidget::tests::plan_mode::plan_slash_command_with_args_submits_prompt_in_plan_mode
        PASS [   0.797s] codex-tui chatwidget::tests::plan_mode::plan_implementation_popup_no_selected_snapshot
        PASS [   0.862s] codex-tui chatwidget::tests::plan_mode::plan_implementation_popup_context_usage_snapshot
  TRY 2 FAIL [   0.916s] codex-tui chatwidget::tests::permissions::permissions_selection_history_snapshot_full_access_to_default
  stdout ---

    running 1 test
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Snapshot Summary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Snapshot file: tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__permissions_selection_history_full_access_to_default@windows.snap
    Snapshot: codex_tui__chatwidget__tests__permissions_selection_history_full_access_to_default@windows
    Source: C:\CodexLab\codex\codex-rs:802
    ───────────────────────────────────────────────────────────────────────────────
    Expression: lines_to_single_string(&cells[0])
    ───────────────────────────────────────────────────────────────────────────────
    -old snapshot
    +new results
    ────────────┬──────────────────────────────────────────────────────────────────
        1       │-• Permissions updated to Ask for approval (non-admin sandbox)
              1 │+ⓘ Permissions updated to Approve for me
    ────────────┴──────────────────────────────────────────────────────────────────
    To update snapshots run `cargo insta review`
    Stopped on the first failure. Run `cargo insta test` to run all snapshots.
    test chatwidget::tests::permissions::permissions_selection_history_snapshot_full_access_to_default ... FAILED

    failures:

    failures:
        chatwidget::tests::permissions::permissions_selection_history_snapshot_full_access_to_default

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.85s

  stderr ---
    stored new snapshot C:\CodexLab\codex\codex-rs\tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__permissions_selection_history_full_access_to_default@windows.snap.new

    thread 'chatwidget::tests::permissions::permissions_selection_history_snapshot_full_access_to_default' (24696) panicked at C:\Users\JohnDell\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\insta-1.46.3\src\runtime.rs:719:13:
    snapshot assertion for 'codex_tui__chatwidget__tests__permissions_selection_history_full_access_to_default@windows' failed in line 802
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.135s] codex-tui chatwidget::tests::plan_mode::plan_update_renders_history_cell
        PASS [   0.119s] codex-tui chatwidget::tests::plan_mode::reasoning_selection_in_plan_mode_matching_plan_effort_but_different_global_opens_scope_prompt
        PASS [   0.124s] codex-tui chatwidget::tests::plan_mode::reasoning_selection_in_plan_mode_model_switch_does_not_open_scope_prompt_event
        PASS [   0.105s] codex-tui chatwidget::tests::plan_mode::reasoning_selection_in_plan_mode_opens_scope_prompt_event
        PASS [   0.116s] codex-tui chatwidget::tests::plan_mode::reasoning_shortcut_in_plan_mode_updates_plan_override_without_prompt_or_persist
        PASS [   0.120s] codex-tui chatwidget::tests::plan_mode::reasoning_selection_in_plan_mode_without_effort_change_does_not_open_scope_prompt_event
        PASS [   0.102s] codex-tui chatwidget::tests::plan_mode::request_user_input_notification_overrides_pending_agent_turn_complete_notification
        PASS [   0.099s] codex-tui chatwidget::tests::plan_mode::set_model_updates_active_collaboration_mask
        PASS [   0.113s] codex-tui chatwidget::tests::plan_mode::set_reasoning_effort_does_not_override_active_plan_override
        PASS [   0.116s] codex-tui chatwidget::tests::plan_mode::submit_user_message_blocks_when_thread_model_is_unavailable
        PASS [   0.120s] codex-tui chatwidget::tests::plan_mode::set_reasoning_effort_updates_active_collaboration_mask
        PASS [   0.126s] codex-tui chatwidget::tests::plan_mode::submit_user_message_emits_structured_plugin_mentions_from_bindings
        PASS [   0.113s] codex-tui chatwidget::tests::plan_mode::submit_user_message_queues_while_compaction_turn_is_running
        PASS [   0.799s] codex-tui chatwidget::tests::plan_mode::plan_implementation_popup_snapshot
        PASS [   0.112s] codex-tui chatwidget::tests::plan_mode::submit_user_message_with_mode_errors_when_mode_changes_during_running_turn
        PASS [   0.113s] codex-tui chatwidget::tests::plan_mode::submit_user_message_with_mode_allows_same_mode_during_running_turn
        PASS [   0.120s] codex-tui chatwidget::tests::plan_mode::submit_user_message_with_mode_sets_coding_collaboration_mode
        PASS [   0.110s] codex-tui chatwidget::tests::plan_mode::submit_user_message_with_mode_submits_when_plan_stream_is_not_active
        PASS [   0.104s] codex-tui chatwidget::tests::plan_mode::user_turn_includes_personality_from_config
        PASS [   0.122s] codex-tui chatwidget::tests::popups_and_settings::apps_notification_update_excludes_inaccessible_apps_from_mentions
        PASS [   0.146s] codex-tui chatwidget::tests::plan_mode::vim_mode_default_disabled_starts_composer_in_insert_mode
        PASS [   0.159s] codex-tui chatwidget::tests::plan_mode::vim_mode_default_enabled_starts_composer_in_normal_mode
        PASS [   0.803s] codex-tui chatwidget::tests::plan_mode::plan_mode_nudge_narrow_snapshot
        PASS [   0.142s] codex-tui chatwidget::tests::popups_and_settings::apps_popup_keeps_existing_full_snapshot_while_partial_refresh_loads
        PASS [   0.163s] codex-tui chatwidget::tests::popups_and_settings::apps_popup_for_not_installed_app_uses_install_only_selected_description
        PASS [   0.149s] codex-tui chatwidget::tests::popups_and_settings::apps_popup_shows_disabled_status_for_installed_but_disabled_apps
        PASS [   0.129s] codex-tui chatwidget::tests::popups_and_settings::apps_refresh_failure_keeps_existing_full_snapshot
        PASS [   0.153s] codex-tui chatwidget::tests::popups_and_settings::apps_popup_preserves_selected_app_across_refresh
        PASS [   0.103s] codex-tui chatwidget::tests::popups_and_settings::apps_refresh_failure_with_cached_snapshot_triggers_pending_force_refetch
        PASS [   0.794s] codex-tui chatwidget::tests::plan_mode::plan_mode_nudge_snapshot
        PASS [   0.131s] codex-tui chatwidget::tests::popups_and_settings::apps_refresh_failure_without_full_snapshot_falls_back_to_installed_apps
        PASS [   0.114s] codex-tui chatwidget::tests::popups_and_settings::deleted_realtime_meter_uses_shared_stop_path
        PASS [   0.117s] codex-tui chatwidget::tests::popups_and_settings::apps_refresh_preserves_toggled_enabled_state
        PASS [   0.117s] codex-tui chatwidget::tests::popups_and_settings::experimental_features_toggle_saves_on_exit
        PASS [   0.140s] codex-tui chatwidget::tests::popups_and_settings::experimental_mode_plan_is_ignored_on_startup
        PASS [   0.122s] codex-tui chatwidget::tests::popups_and_settings::experimental_popup_omits_stable_guardian_approval
        PASS [   0.110s] codex-tui chatwidget::tests::popups_and_settings::memories_enable_prompt_updates_feature_without_notice
        PASS [   0.107s] codex-tui chatwidget::tests::popups_and_settings::memories_reset_confirmation_sends_event_on_confirm
        PASS [   0.867s] codex-tui chatwidget::tests::popups_and_settings::apps_popup_stays_loading_until_final_snapshot_updates
        PASS [   0.780s] codex-tui chatwidget::tests::popups_and_settings::experimental_features_popup_snapshot
        PASS [   0.851s] codex-tui chatwidget::tests::popups_and_settings::feedback_good_result_consent_popup_includes_connectivity_diagnostics_filename
        PASS [   0.115s] codex-tui chatwidget::tests::popups_and_settings::memories_settings_toggle_saves_on_enter
        PASS [   0.853s] codex-tui chatwidget::tests::popups_and_settings::feedback_upload_consent_popup_snapshot
        PASS [   0.898s] codex-tui chatwidget::tests::popups_and_settings::feedback_selection_popup_snapshot
        PASS [   0.876s] codex-tui chatwidget::tests::popups_and_settings::marketplace_add_success_refreshes_to_new_marketplace_tab
        PASS [   0.918s] codex-tui chatwidget::tests::popups_and_settings::hooks_popup_shows_list_diagnostics
        PASS [   0.090s] codex-tui chatwidget::tests::popups_and_settings::model_reasoning_selection_popup_applies_custom_effort
        PASS [   0.874s] codex-tui chatwidget::tests::popups_and_settings::marketplace_upgrade_failure_includes_backend_messages_snapshot
        PASS [   0.932s] codex-tui chatwidget::tests::popups_and_settings::marketplace_upgrade_loading_popup_snapshot
        PASS [   0.746s] codex-tui chatwidget::tests::popups_and_settings::memories_settings_popup_snapshot
        PASS [   0.930s] codex-tui chatwidget::tests::popups_and_settings::memories_enable_prompt_snapshot
        PASS [   0.125s] codex-tui chatwidget::tests::popups_and_settings::multi_agent_enable_prompt_updates_feature_and_emits_notice
        PASS [   0.809s] codex-tui chatwidget::tests::popups_and_settings::memories_reset_confirmation_snapshot
        PASS [   0.145s] codex-tui chatwidget::tests::popups_and_settings::plugins_popup_add_marketplace_tab_opens_prompt_and_submits_source
        PASS [   0.133s] codex-tui chatwidget::tests::popups_and_settings::plugins_popup_installed_tab_filters_rows_and_clears_search
        PASS [   0.127s] codex-tui chatwidget::tests::popups_and_settings::plugins_popup_openai_curated_tab_omits_marketplace_in_rows
        PASS [   0.110s] codex-tui chatwidget::tests::popups_and_settings::plugins_popup_refresh_preserves_duplicate_marketplace_tab_by_path
        PASS [   0.229s] codex-tui chatwidget::tests::popups_and_settings::plugins_popup_refresh_preserves_selected_row_position
        PASS [   0.229s] codex-tui chatwidget::tests::popups_and_settings::plugins_popup_refreshes_installed_counts_after_install
        PASS [   0.822s] codex-tui chatwidget::tests::popups_and_settings::model_reasoning_selection_popup_extra_high_warning_snapshot
        PASS [   0.920s] codex-tui chatwidget::tests::popups_and_settings::model_picker_hides_show_in_picker_false_models_from_cache
        PASS [   0.835s] codex-tui chatwidget::tests::popups_and_settings::model_selection_popup_snapshot
        PASS [   0.840s] codex-tui chatwidget::tests::popups_and_settings::multi_agent_enable_prompt_snapshot
        PASS [   0.863s] codex-tui chatwidget::tests::popups_and_settings::plugin_detail_error_popup_skips_disabled_row_numbering
        PASS [   0.138s] codex-tui chatwidget::tests::popups_and_settings::plugins_popup_search_no_matches_and_backspace_restores_results
        PASS [   0.971s] codex-tui chatwidget::tests::popups_and_settings::model_reasoning_selection_popup_snapshot
        PASS [   0.875s] codex-tui chatwidget::tests::popups_and_settings::plugin_detail_popup_snapshot_shows_install_actions_and_capability_summaries
        PASS [   0.947s] codex-tui chatwidget::tests::popups_and_settings::personality_selection_popup_snapshot
        PASS [   0.894s] codex-tui chatwidget::tests::popups_and_settings::plugin_detail_popup_hides_disclosure_for_installed_plugins
        PASS [   0.116s] codex-tui chatwidget::tests::popups_and_settings::plugins_popup_space_with_active_search_does_not_toggle_installed_plugin
        PASS [   0.883s] codex-tui chatwidget::tests::popups_and_settings::plugins_popup_loading_state_snapshot
        PASS [   0.151s] codex-tui chatwidget::tests::popups_and_settings::plugins_popup_space_on_uninstalled_row_does_not_start_search
        PASS [   0.156s] codex-tui chatwidget::tests::popups_and_settings::plugins_popup_space_toggles_installed_plugin_from_list
        PASS [   0.105s] codex-tui chatwidget::tests::popups_and_settings::realtime_audio_picker_emits_persist_event
        PASS [   0.170s] codex-tui chatwidget::tests::popups_and_settings::plugins_popup_upgrades_user_configured_git_marketplace_from_marketplace_tab
        PASS [   0.110s] codex-tui chatwidget::tests::popups_and_settings::reasoning_down_shortcuts_lower_reasoning_effort
        PASS [   0.120s] codex-tui chatwidget::tests::popups_and_settings::reasoning_popup_shows_extra_high_with_space
        PASS [   0.121s] codex-tui chatwidget::tests::popups_and_settings::reasoning_shortcut_clears_armed_quit_shortcut
        PASS [   0.158s] codex-tui chatwidget::tests::popups_and_settings::reasoning_popup_escape_returns_to_model_popup
        PASS [   0.110s] codex-tui chatwidget::tests::popups_and_settings::reasoning_shortcut_is_ignored_with_model_popup_open
        PASS [   0.099s] codex-tui chatwidget::tests::popups_and_settings::server_overloaded_error_does_not_switch_models
        PASS [   0.135s] codex-tui chatwidget::tests::popups_and_settings::reasoning_up_shortcuts_raise_reasoning_effort
        PASS [   0.140s] codex-tui chatwidget::tests::popups_and_settings::single_reasoning_option_skips_selection
        PASS [   0.105s] codex-tui chatwidget::tests::review_mode::ctrl_c_cleared_prompt_is_recoverable_via_history
        PASS [   0.187s] codex-tui chatwidget::tests::review_mode::budget_limited_turn_restores_queued_input_without_submitting
        PASS [   0.140s] codex-tui chatwidget::tests::review_mode::ctrl_c_closes_realtime_conversation_before_interrupt_or_quit
        PASS [   0.131s] codex-tui chatwidget::tests::review_mode::ctrl_c_interrupts_without_arming_quit_when_double_press_disabled
        PASS [   0.859s] codex-tui chatwidget::tests::popups_and_settings::plugins_popup_search_filters_visible_rows_snapshot
        PASS [   0.128s] codex-tui chatwidget::tests::review_mode::ctrl_c_shutdown_works_with_caps_lock
        PASS [   0.890s] codex-tui chatwidget::tests::popups_and_settings::plugins_popup_removes_user_configured_marketplace_flow
        PASS [   0.814s] codex-tui chatwidget::tests::popups_and_settings::plugins_popup_snapshot_shows_all_marketplaces_and_sorts_installed_then_name
        PASS [   0.110s] codex-tui chatwidget::tests::review_mode::custom_prompt_enter_empty_does_not_send
        PASS [   0.127s] codex-tui chatwidget::tests::review_mode::custom_prompt_submit_sends_review_op
        PASS [   0.116s] codex-tui chatwidget::tests::review_mode::enter_submits_steer_while_review_is_running
        PASS [   0.114s] codex-tui chatwidget::tests::review_mode::entered_review_mode_defaults_to_current_changes_banner
        PASS [   0.117s] codex-tui chatwidget::tests::review_mode::entered_review_mode_uses_request_hint
        PASS [   0.107s] codex-tui chatwidget::tests::review_mode::esc_with_pending_steers_overrides_agent_command_interrupt_behavior
        PASS [   0.132s] codex-tui chatwidget::tests::review_mode::esc_interrupt_sends_all_pending_steers_immediately_and_keeps_existing_draft
        PASS [   0.111s] codex-tui chatwidget::tests::review_mode::failed_pending_steer_submit_does_not_add_pending_preview
        PASS [   0.876s] codex-tui chatwidget::tests::popups_and_settings::plugins_popup_truncates_long_descriptions_in_list_rows
        PASS [   0.816s] codex-tui chatwidget::tests::popups_and_settings::realtime_audio_selection_popup_narrow_snapshot
        PASS [   0.108s] codex-tui chatwidget::tests::review_mode::interrupted_turn_restores_queued_messages_with_images_and_elements
        PASS [   0.850s] codex-tui chatwidget::tests::popups_and_settings::realtime_audio_selection_popup_snapshot
        PASS [   0.100s] codex-tui chatwidget::tests::review_mode::item_completed_only_pops_front_pending_steer
        PASS [   0.870s] codex-tui chatwidget::tests::popups_and_settings::realtime_error_closes_without_followup_closed_info
        PASS [   0.892s] codex-tui chatwidget::tests::popups_and_settings::realtime_microphone_picker_popup_snapshot
        PASS [   0.110s] codex-tui chatwidget::tests::review_mode::live_app_server_review_prompt_item_is_not_rendered
        PASS [   0.125s] codex-tui chatwidget::tests::review_mode::item_completed_pops_pending_steer_with_local_image_and_text_elements
        PASS [   0.125s] codex-tui chatwidget::tests::review_mode::live_agent_message_renders_during_review_mode
        PASS [   0.124s] codex-tui chatwidget::tests::review_mode::live_review_prompt_item_is_not_rendered
        PASS [   0.123s] codex-tui chatwidget::tests::review_mode::manual_interrupt_restores_pending_steer_mention_bindings_to_composer
        PASS [   0.122s] codex-tui chatwidget::tests::review_mode::manual_interrupt_restores_pending_steers_before_queued_messages
        PASS [   0.125s] codex-tui chatwidget::tests::review_mode::restore_thread_input_state_restores_pending_steers_without_downgrading_them
        PASS [   0.125s] codex-tui chatwidget::tests::review_mode::manual_interrupt_restores_pending_steers_to_composer
        PASS [   0.107s] codex-tui chatwidget::tests::review_mode::review_popup_custom_prompt_action_sends_event
        PASS [   0.156s] codex-tui chatwidget::tests::review_mode::review_commit_picker_shows_subjects_without_timestamps
        PASS [   0.174s] codex-tui chatwidget::tests::review_mode::review_custom_prompt_escape_navigates_back_then_dismisses
        PASS [   0.114s] codex-tui chatwidget::tests::review_mode::review_restores_context_window_indicator
        PASS [   0.143s] codex-tui chatwidget::tests::review_mode::steer_enter_during_final_stream_preserves_follow_up_prompts_in_order
        PASS [   0.116s] codex-tui chatwidget::tests::review_mode::steer_enter_queues_while_plan_stream_is_active
        PASS [   0.157s] codex-tui chatwidget::tests::review_mode::steer_enter_uses_pending_steers_while_final_answer_stream_is_active
        PASS [   0.134s] codex-tui chatwidget::tests::review_mode::steer_enter_uses_pending_steers_while_turn_is_running_without_streaming
        PASS [   0.784s] codex-tui chatwidget::tests::review_mode::direct_budget_limited_turn_uses_budget_message_snapshot
        PASS [   0.119s] codex-tui chatwidget::tests::review_mode::steer_rejection_queues_review_follow_up_before_existing_queued_messages
        PASS [   0.142s] codex-tui chatwidget::tests::side::slash_btw_is_rejected_before_the_session_starts
        PASS [   0.162s] codex-tui chatwidget::tests::side::slash_btw_is_rejected_during_review_mode
        PASS [   0.154s] codex-tui chatwidget::tests::side::slash_btw_requests_forked_side_question_while_task_running
        PASS [   0.815s] codex-tui chatwidget::tests::review_mode::interrupted_turn_after_goal_budget_limited_uses_budget_message_snapshot
        PASS [   0.842s] codex-tui chatwidget::tests::review_mode::interrupted_turn_error_message_snapshot
        PASS [   0.579s] codex-tui chatwidget::tests::review_mode::review_branch_picker_escape_navigates_back_then_dismisses
        PASS [   0.098s] codex-tui chatwidget::tests::side::slash_commands_without_side_flag_are_rejected_for_side_threads
        PASS [   0.970s] codex-tui chatwidget::tests::review_mode::interrupt_exec_marks_failed_snapshot
        PASS [   0.151s] codex-tui chatwidget::tests::side::slash_rename_is_rejected_for_side_threads
        PASS [   0.152s] codex-tui chatwidget::tests::side::slash_rename_with_args_is_rejected_for_side_threads
        PASS [   0.157s] codex-tui chatwidget::tests::side::slash_btw_without_args_starts_empty_side_conversation
        PASS [   0.117s] codex-tui chatwidget::tests::side::slash_side_is_rejected_for_side_threads
  TRY 1 FAIL [   0.966s] codex-tui chatwidget::tests::review_mode::interrupted_turn_pending_steers_message_snapshot
  stdout ---

    running 1 test
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Snapshot Summary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Snapshot file: tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__interrupted_turn_pending_steers_message.snap
    Snapshot: codex_tui__chatwidget__tests__interrupted_turn_pending_steers_message
    Source: C:\CodexLab\codex\codex-rs:1325
    ───────────────────────────────────────────────────────────────────────────────
    Expression: info
    ───────────────────────────────────────────────────────────────────────────────
    -old snapshot
    +new results
    ────────────┬──────────────────────────────────────────────────────────────────
        1       │-• Model interrupted to submit steer instructions.
              1 │+ⓘ Model interrupted to submit steer instructions.
    ────────────┴──────────────────────────────────────────────────────────────────
    To update snapshots run `cargo insta review`
    Stopped on the first failure. Run `cargo insta test` to run all snapshots.
    test chatwidget::tests::review_mode::interrupted_turn_pending_steers_message_snapshot ... FAILED

    failures:

    failures:
        chatwidget::tests::review_mode::interrupted_turn_pending_steers_message_snapshot

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.91s

  stderr ---
    stored new snapshot C:\CodexLab\codex\codex-rs\tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__interrupted_turn_pending_steers_message.snap.new

    thread 'chatwidget::tests::review_mode::interrupted_turn_pending_steers_message_snapshot' (24124) panicked at C:\Users\JohnDell\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\insta-1.46.3\src\runtime.rs:719:13:
    snapshot assertion for 'codex_tui__chatwidget__tests__interrupted_turn_pending_steers_message' failed in line 1325
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.145s] codex-tui chatwidget::tests::side::slash_side_is_rejected_during_review_mode
        PASS [   0.116s] codex-tui chatwidget::tests::side::slash_side_without_args_starts_empty_side_conversation
        PASS [   0.124s] codex-tui chatwidget::tests::side::suppressed_interrupted_turn_notice_skips_history_warning
        PASS [   0.124s] codex-tui chatwidget::tests::side::submit_user_message_as_plain_user_turn_does_not_run_shell_commands
        PASS [   0.153s] codex-tui chatwidget::tests::slash_commands::active_goal_without_follow_up_suppresses_agent_turn_complete_notification
        PASS [   0.127s] codex-tui chatwidget::tests::slash_commands::bare_goal_slash_command_drains_pending_submission_state
        PASS [   0.152s] codex-tui chatwidget::tests::slash_commands::agent_turn_complete_notification_does_not_reuse_stale_copy_source
        PASS [   0.105s] codex-tui chatwidget::tests::slash_commands::bare_slash_command_is_available_from_local_recall_after_dispatch
        PASS [   0.887s] codex-tui chatwidget::tests::review_mode::review_queues_user_messages_snapshot
        PASS [   0.130s] codex-tui chatwidget::tests::slash_commands::copy_shortcut_can_be_remapped
        PASS [   0.130s] codex-tui chatwidget::tests::slash_commands::ctrl_d_quits_without_prompt
        PASS [   0.129s] codex-tui chatwidget::tests::slash_commands::ctrl_d_with_modal_open_does_not_quit
        PASS [   0.146s] codex-tui chatwidget::tests::slash_commands::ctrl_o_copy_reports_when_no_agent_response_exists
        PASS [   0.112s] codex-tui chatwidget::tests::slash_commands::fast_keybinding_toggle_requires_feature_and_idle_surface
        PASS [   0.787s] codex-tui chatwidget::tests::side::forked_thread_history_line_without_name_shows_id_once_snapshot
        PASS [   0.091s] codex-tui chatwidget::tests::slash_commands::fast_keybinding_toggle_uses_same_events_as_fast_slash_command
        PASS [   0.109s] codex-tui chatwidget::tests::slash_commands::fast_slash_command_updates_and_persists_local_service_tier
        PASS [   0.855s] codex-tui chatwidget::tests::side::side_context_label_preserves_status_line_snapshot
        PASS [   0.123s] codex-tui chatwidget::tests::slash_commands::goal_edit_slash_command_opens_goal_editor
        PASS [   0.152s] codex-tui chatwidget::tests::slash_commands::goal_control_slash_commands_emit_goal_events
        PASS [   0.124s] codex-tui chatwidget::tests::slash_commands::goal_slash_command_drops_attached_images
        PASS [   0.118s] codex-tui chatwidget::tests::slash_commands::goal_slash_command_emits_set_goal_event
        PASS [   0.123s] codex-tui chatwidget::tests::slash_commands::goal_slash_command_uses_plain_text_for_mentions
        PASS [   0.881s] codex-tui chatwidget::tests::side::side_context_label_shows_parent_status_snapshot
        PASS [   0.074s] codex-tui chatwidget::tests::slash_commands::merged_history_record_preserves_raw_text_and_rebased_elements
        PASS [   0.060s] codex-tui chatwidget::tests::slash_commands::merged_history_record_remaps_override_image_placeholders
        PASS [   0.098s] codex-tui chatwidget::tests::slash_commands::inline_slash_command_is_available_from_local_recall_after_dispatch
        PASS [   0.112s] codex-tui chatwidget::tests::slash_commands::interrupted_merged_message_history_encodes_mentions_once
        PASS [   0.122s] codex-tui chatwidget::tests::slash_commands::keymap_capture_can_capture_current_copy_shortcut
        PASS [   0.112s] codex-tui chatwidget::tests::slash_commands::no_op_stub_slash_command_is_available_from_local_recall
        PASS [   0.118s] codex-tui chatwidget::tests::slash_commands::model_switch_recomputes_catalog_default_service_tier
        PASS [   0.110s] codex-tui chatwidget::tests::slash_commands::queued_bang_shell_dispatches_after_active_turn
        PASS [   0.115s] codex-tui chatwidget::tests::slash_commands::queued_bang_shell_waits_for_user_shell_completion_before_next_input
        PASS [   0.127s] codex-tui chatwidget::tests::slash_commands::queued_empty_bang_shell_reports_help_when_dequeued_and_drains_next_input
        PASS [   0.140s] codex-tui chatwidget::tests::slash_commands::queued_bare_rename_drains_next_input_after_name_update
        PASS [   0.132s] codex-tui chatwidget::tests::slash_commands::queued_fast_slash_applies_before_next_queued_message
        PASS [   0.137s] codex-tui chatwidget::tests::slash_commands::queued_follow_up_suppresses_agent_turn_complete_notification
        PASS [   0.100s] codex-tui chatwidget::tests::slash_commands::queued_goal_slash_command_emits_set_goal_event_after_thread_starts
        PASS [   0.759s] codex-tui chatwidget::tests::side::slash_side_requests_forked_side_question_while_task_running
        PASS [   0.186s] codex-tui chatwidget::tests::slash_commands::queued_goal_slash_command_preserves_current_draft_metadata
        PASS [   0.183s] codex-tui chatwidget::tests::slash_commands::queued_inline_rename_does_not_drain_again_before_turn_started
        PASS [   0.190s] codex-tui chatwidget::tests::slash_commands::queued_menu_slash_keeps_agent_turn_complete_notification
        PASS [   0.176s] codex-tui chatwidget::tests::slash_commands::queued_slash_compact_dispatches_after_active_turn
        PASS [   0.160s] codex-tui chatwidget::tests::slash_commands::queued_slash_review_with_args_restores_for_edit
        PASS [   0.172s] codex-tui chatwidget::tests::slash_commands::queued_slash_review_with_args_dispatches_after_active_turn
        PASS [   0.182s] codex-tui chatwidget::tests::slash_commands::queued_slash_menu_selection_drains_next_input
        PASS [   0.219s] codex-tui chatwidget::tests::slash_commands::queued_slash_menu_cancel_drains_next_input
        PASS [   0.188s] codex-tui chatwidget::tests::slash_commands::queued_unknown_slash_reports_error_when_dequeued
        PASS [   0.100s] codex-tui chatwidget::tests::slash_commands::raw_slash_command_reports_usage_for_invalid_arg
  TRY 2 FAIL [   0.895s] codex-tui chatwidget::tests::review_mode::interrupted_turn_pending_steers_message_snapshot
  stdout ---

    running 1 test
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Snapshot Summary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Snapshot file: tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__interrupted_turn_pending_steers_message.snap
    Snapshot: codex_tui__chatwidget__tests__interrupted_turn_pending_steers_message
    Source: C:\CodexLab\codex\codex-rs:1325
    ───────────────────────────────────────────────────────────────────────────────
    Expression: info
    ───────────────────────────────────────────────────────────────────────────────
    -old snapshot
    +new results
    ────────────┬──────────────────────────────────────────────────────────────────
        1       │-• Model interrupted to submit steer instructions.
              1 │+ⓘ Model interrupted to submit steer instructions.
    ────────────┴──────────────────────────────────────────────────────────────────
    To update snapshots run `cargo insta review`
    Stopped on the first failure. Run `cargo insta test` to run all snapshots.
    test chatwidget::tests::review_mode::interrupted_turn_pending_steers_message_snapshot ... FAILED

    failures:

    failures:
        chatwidget::tests::review_mode::interrupted_turn_pending_steers_message_snapshot

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.83s

  stderr ---
    stored new snapshot C:\CodexLab\codex\codex-rs\tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__interrupted_turn_pending_steers_message.snap.new

    thread 'chatwidget::tests::review_mode::interrupted_turn_pending_steers_message_snapshot' (25688) panicked at C:\Users\JohnDell\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\insta-1.46.3\src\runtime.rs:719:13:
    snapshot assertion for 'codex_tui__chatwidget__tests__interrupted_turn_pending_steers_message' failed in line 1325
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.107s] codex-tui chatwidget::tests::slash_commands::service_tier_commands_lowercase_catalog_names
        PASS [   0.808s] codex-tui chatwidget::tests::slash_commands::compact_queues_user_messages_snapshot
        PASS [   0.121s] codex-tui chatwidget::tests::slash_commands::raw_slash_command_toggles_and_accepts_on_off_args
        PASS [   0.123s] codex-tui chatwidget::tests::slash_commands::slash_app_requests_desktop_handoff
        PASS [   0.146s] codex-tui chatwidget::tests::slash_commands::restored_queued_goal_slash_command_emits_set_goal_event
        PASS [   0.131s] codex-tui chatwidget::tests::slash_commands::slash_archive_is_disabled_while_task_running
        PASS [   0.139s] codex-tui chatwidget::tests::slash_commands::slash_clear_is_disabled_while_task_running
        PASS [   0.155s] codex-tui chatwidget::tests::slash_commands::slash_clear_after_ctrl_c_keeps_stashed_draft_recallable
        PASS [   0.126s] codex-tui chatwidget::tests::slash_commands::slash_clear_requests_ui_clear_when_idle
        PASS [   0.109s] codex-tui chatwidget::tests::slash_commands::slash_compact_eagerly_queues_follow_up_before_turn_start
        PASS [   0.794s] codex-tui chatwidget::tests::slash_commands::goal_control_slash_command_without_thread_shows_full_usage
        PASS [   0.134s] codex-tui chatwidget::tests::slash_commands::slash_copy_reports_when_rewind_exceeds_retained_copy_history
        PASS [   0.110s] codex-tui chatwidget::tests::slash_commands::slash_copy_state_tracks_plan_item_completion
        PASS [   0.120s] codex-tui chatwidget::tests::slash_commands::slash_copy_state_is_preserved_during_running_task
        PASS [   0.123s] codex-tui chatwidget::tests::slash_commands::slash_copy_state_tracks_turn_complete_final_reply
        PASS [   0.126s] codex-tui chatwidget::tests::slash_commands::slash_copy_stores_clipboard_lease_and_preserves_it_on_failure
        PASS [   0.114s] codex-tui chatwidget::tests::slash_commands::slash_exit_requests_exit
        PASS [   0.122s] codex-tui chatwidget::tests::slash_commands::slash_copy_uses_agent_message_item_when_turn_complete_omits_final_text
        PASS [   0.131s] codex-tui chatwidget::tests::slash_commands::slash_copy_uses_latest_surviving_response_after_rollback
        PASS [   0.129s] codex-tui chatwidget::tests::slash_commands::slash_fork_requests_current_fork
        PASS [   0.134s] codex-tui chatwidget::tests::slash_commands::slash_keymap_debug_can_inspect_app_shortcuts
        PASS [   0.149s] codex-tui chatwidget::tests::slash_commands::slash_keymap_capture_can_capture_app_shortcuts
        PASS [   0.165s] codex-tui chatwidget::tests::slash_commands::slash_init_skips_when_project_doc_exists
        PASS [   0.147s] codex-tui chatwidget::tests::slash_commands::slash_keymap_debug_opens_keypress_inspector
        PASS [   0.128s] codex-tui chatwidget::tests::slash_commands::slash_keymap_invalid_args_show_usage
        PASS [   0.129s] codex-tui chatwidget::tests::slash_commands::slash_logout_requests_app_server_logout
        PASS [   0.125s] codex-tui chatwidget::tests::slash_commands::slash_mcp_invalid_args_show_usage
        PASS [   0.119s] codex-tui chatwidget::tests::slash_commands::slash_mcp_requests_inventory_via_app_server
        PASS [   0.107s] codex-tui chatwidget::tests::slash_commands::slash_mcp_verbose_requests_full_inventory_via_app_server
        PASS [   0.117s] codex-tui chatwidget::tests::slash_commands::slash_memories_opens_memory_menu
        PASS [   0.083s] codex-tui chatwidget::tests::slash_commands::slash_memory_drop_reports_stubbed_feature
        PASS [   0.099s] codex-tui chatwidget::tests::slash_commands::slash_memory_update_reports_stubbed_feature
        PASS [   0.103s] codex-tui chatwidget::tests::slash_commands::slash_pets_disable_disables_pets_even_on_unsupported_terminal
        PASS [   0.112s] codex-tui chatwidget::tests::slash_commands::slash_pet_hide_disables_pets_even_on_unsupported_terminal
        PASS [   0.146s] codex-tui chatwidget::tests::slash_commands::slash_pets_on_old_iterm2_shows_upgrade_warning
        PASS [   0.142s] codex-tui chatwidget::tests::slash_commands::slash_pets_on_unsupported_terminal_shows_terminal_warning
        PASS [   0.154s] codex-tui chatwidget::tests::slash_commands::slash_pets_on_unsupported_terminal_warns_without_picker
        PASS [   0.117s] codex-tui chatwidget::tests::slash_commands::slash_pets_with_arg_on_unsupported_terminal_warns_without_selection
        PASS [   0.112s] codex-tui chatwidget::tests::slash_commands::slash_pets_with_arg_selects_named_pet
        PASS [   0.114s] codex-tui chatwidget::tests::slash_commands::slash_rename_without_existing_thread_name_starts_empty
        PASS [   0.132s] codex-tui chatwidget::tests::slash_commands::slash_quit_requests_exit
        PASS [   0.107s] codex-tui chatwidget::tests::slash_commands::slash_resume_with_arg_requests_named_session
        PASS [   0.097s] codex-tui chatwidget::tests::slash_commands::slash_resume_opens_picker
        PASS [   0.110s] codex-tui chatwidget::tests::slash_commands::slash_rollout_displays_current_path
        PASS [   0.135s] codex-tui chatwidget::tests::slash_commands::slash_rollout_handles_missing_path
        PASS [   0.107s] codex-tui chatwidget::tests::slash_commands::slash_stop_submits_background_terminal_cleanup
        PASS [   0.117s] codex-tui chatwidget::tests::slash_commands::unavailable_slash_command_is_available_from_local_recall
        PASS [   0.117s] codex-tui chatwidget::tests::slash_commands::usage_error_slash_command_is_available_from_local_recall
        PASS [   0.822s] codex-tui chatwidget::tests::slash_commands::slash_app_without_thread_id_shows_starting_error
        PASS [   0.129s] codex-tui chatwidget::tests::slash_commands::user_turn_carries_service_tier_after_fast_toggle
        PASS [   0.173s] codex-tui chatwidget::tests::slash_commands::unrecognized_slash_command_is_not_added_to_local_recall
        PASS [   0.130s] codex-tui chatwidget::tests::slash_commands::user_turn_sends_standard_override_after_fast_is_turned_off
        PASS [   0.130s] codex-tui chatwidget::tests::status_and_layout::ambient_pet_can_be_disabled
        PASS [   0.880s] codex-tui chatwidget::tests::slash_commands::slash_archive_confirmation_requests_current_thread_archive
        PASS [   0.132s] codex-tui chatwidget::tests::status_and_layout::ambient_pet_draw_uses_terminal_screen_area_not_short_inline_viewport
        PASS [   0.105s] codex-tui chatwidget::tests::status_and_layout::ambient_pet_reserves_history_wrap_width
        PASS [   0.841s] codex-tui chatwidget::tests::slash_commands::slash_copy_reports_when_no_agent_response_exists
        PASS [   0.137s] codex-tui chatwidget::tests::status_and_layout::ambient_pet_hides_notification_text_overlay
        PASS [   0.134s] codex-tui chatwidget::tests::status_and_layout::app_server_cyber_policy_error_renders_dedicated_notice
        PASS [   0.167s] codex-tui chatwidget::tests::status_and_layout::ambient_pet_screen_bottom_anchor_uses_terminal_bottom
        PASS [   0.173s] codex-tui chatwidget::tests::status_and_layout::ambient_pet_reduces_stream_width_and_composer_text_width
        PASS [   0.129s] codex-tui chatwidget::tests::status_and_layout::app_server_model_verification_renders_warning
        PASS [   0.137s] codex-tui chatwidget::tests::status_and_layout::commentary_completion_restores_status_indicator_before_exec_begin
        PASS [   0.136s] codex-tui chatwidget::tests::status_and_layout::completed_plan_table_tail_skips_provisional_history_insert
        PASS [   0.126s] codex-tui chatwidget::tests::status_and_layout::context_indicator_shows_used_tokens_when_window_unknown
        PASS [   0.878s] codex-tui chatwidget::tests::slash_commands::slash_pets_opens_picker
        PASS [   0.799s] codex-tui chatwidget::tests::slash_commands::slash_rename_prefills_existing_thread_name
        PASS [   0.150s] codex-tui chatwidget::tests::status_and_layout::ctrl_c_interrupt_pauses_active_goal_turn
        PASS [   0.110s] codex-tui chatwidget::tests::status_and_layout::fast_status_indicator_is_hidden_for_models_without_fast_support
        PASS [   0.110s] codex-tui chatwidget::tests::status_and_layout::fast_status_indicator_is_hidden_when_fast_mode_is_off
        PASS [   0.127s] codex-tui chatwidget::tests::status_and_layout::fast_status_indicator_requires_chatgpt_auth
        PASS [   0.801s] codex-tui chatwidget::tests::status_and_layout::blocked_and_failed_hooks_render_feedback_and_errors
        PASS [   0.161s] codex-tui chatwidget::tests::status_and_layout::final_answer_completion_restores_status_indicator_for_pending_steer
        PASS [   0.774s] codex-tui chatwidget::tests::status_and_layout::chatwidget_tall
        PASS [   0.804s] codex-tui chatwidget::tests::status_and_layout::chatwidget_exec_and_status_layout_vt100_snapshot
        PASS [   0.070s] codex-tui chatwidget::tests::status_and_layout::goal_status_indicator_formats_statuses_and_budgets
        PASS [   0.063s] codex-tui chatwidget::tests::status_and_layout::goal_status_indicator_line_formats_goal_text
        PASS [   0.824s] codex-tui chatwidget::tests::status_and_layout::completed_hook_with_output_flushes_immediately
        PASS [   0.128s] codex-tui chatwidget::tests::status_and_layout::flush_answer_stream_keeps_default_reflow_for_plain_text_tail
        PASS [   0.845s] codex-tui chatwidget::tests::status_and_layout::completed_hook_output_precedes_following_assistant_message
        PASS [   0.134s] codex-tui chatwidget::tests::status_and_layout::flush_answer_stream_requests_scrollback_reflow_for_live_table_tail
        PASS [   0.846s] codex-tui chatwidget::tests::status_and_layout::completed_hook_with_no_entries_stays_out_of_history
        PASS [   0.789s] codex-tui chatwidget::tests::status_and_layout::completed_same_id_hook_output_survives_restart
        PASS [   0.168s] codex-tui chatwidget::tests::status_and_layout::header_rate_limit_snapshot_preserves_member_limit_type_for_error_prompt
        PASS [   1.030s] codex-tui chatwidget::tests::status_and_layout::chatwidget_markdown_code_blocks_vt100_snapshot
        PASS [   0.163s] codex-tui chatwidget::tests::status_and_layout::idle_commit_ticks_do_not_restore_status_without_commentary_completion
        PASS [   0.155s] codex-tui chatwidget::tests::status_and_layout::multiple_agent_messages_in_single_turn_emit_multiple_headers
        PASS [   0.212s] codex-tui chatwidget::tests::status_and_layout::missing_rate_limit_reached_type_does_not_prompt_or_refresh
        PASS [   0.290s] codex-tui chatwidget::tests::status_and_layout::hidden_active_hook_does_not_add_transcript_separator
        PASS [   0.793s] codex-tui chatwidget::tests::status_and_layout::deltas_then_same_final_message_are_rendered_snapshot
        PASS [   0.103s] codex-tui chatwidget::tests::status_and_layout::prefetch_rate_limits_is_gated_on_chatgpt_auth_provider
        PASS [   0.121s] codex-tui chatwidget::tests::status_and_layout::quiet_hook_linger_starts_when_delayed_redraw_reveals_hook
        PASS [   0.065s] codex-tui chatwidget::tests::status_and_layout::rate_limit_duration_labels_only_render_supported_windows
        PASS [   0.128s] codex-tui chatwidget::tests::status_and_layout::rate_limit_snapshot_updates_and_retains_plan_type
        PASS [   0.141s] codex-tui chatwidget::tests::status_and_layout::rate_limit_snapshot_keeps_prior_credits_when_missing_from_headers
        PASS [   0.111s] codex-tui chatwidget::tests::status_and_layout::rate_limit_snapshots_keep_separate_entries_per_limit_id
        PASS [   0.128s] codex-tui chatwidget::tests::status_and_layout::rate_limit_switch_prompt_defers_until_task_complete
        PASS [   0.118s] codex-tui chatwidget::tests::status_and_layout::rate_limit_switch_prompt_respects_hidden_notice
        PASS [   0.780s] codex-tui chatwidget::tests::status_and_layout::final_reasoning_then_message_without_deltas_are_rendered
        PASS [   0.127s] codex-tui chatwidget::tests::status_and_layout::rate_limit_switch_prompt_shows_once_per_session
        PASS [   0.101s] codex-tui chatwidget::tests::status_and_layout::rate_limit_switch_prompt_skips_non_codex_limit
        PASS [   0.070s] codex-tui chatwidget::tests::status_and_layout::rate_limit_warnings_emit_thresholds
        PASS [   0.146s] codex-tui chatwidget::tests::status_and_layout::rate_limit_switch_prompt_skips_when_on_lower_cost_model
        PASS [   0.122s] codex-tui chatwidget::tests::status_and_layout::raw_output_status_line_value_only_shows_when_enabled
        PASS [   0.153s] codex-tui chatwidget::tests::status_and_layout::raw_output_mode_can_change_without_inserting_notice
        PASS [   0.815s] codex-tui chatwidget::tests::status_and_layout::identical_parallel_running_hooks_collapse_to_count
        PASS [   0.125s] codex-tui chatwidget::tests::status_and_layout::repeated_generic_warning_is_not_hidden
        PASS [   0.864s] codex-tui chatwidget::tests::status_and_layout::hook_completed_before_reveal_renders_completed_without_running_flash
        PASS [   0.126s] codex-tui chatwidget::tests::status_and_layout::repeated_model_metadata_warning_is_hidden_for_same_slug
        PASS [   0.881s] codex-tui chatwidget::tests::status_and_layout::interrupted_turn_clears_visible_running_hook
        PASS [   0.135s] codex-tui chatwidget::tests::status_and_layout::rolling_rate_limit_snapshot_preserves_prior_individual_limit
        PASS [   0.790s] codex-tui chatwidget::tests::status_and_layout::overlapping_hook_live_cell_tracks_parallel_quiet_hooks
        PASS [   0.794s] codex-tui chatwidget::tests::status_and_layout::post_tool_use_hook_events_render_snapshot
        PASS [   0.096s] codex-tui chatwidget::tests::status_and_layout::session_configured_clears_goal_status_footer
        PASS [   0.130s] codex-tui chatwidget::tests::status_and_layout::runtime_metrics_websocket_timing_logs_and_final_separator_sums_totals
        PASS [   0.817s] codex-tui chatwidget::tests::status_and_layout::pre_tool_use_hook_events_render_snapshot
        PASS [   0.122s] codex-tui chatwidget::tests::status_and_layout::stale_status_line_git_summary_update_is_ignored
        PASS [   0.136s] codex-tui chatwidget::tests::status_and_layout::status_line_and_terminal_title_reasoning_render_only_effort
        PASS [   0.148s] codex-tui chatwidget::tests::status_and_layout::status_line_branch_changes_render_no_changes
        PASS [   0.129s] codex-tui chatwidget::tests::status_and_layout::status_line_branch_refreshes_after_turn_complete
        PASS [   0.150s] codex-tui chatwidget::tests::status_and_layout::status_line_branch_refreshes_after_interrupt
        PASS [   0.131s] codex-tui chatwidget::tests::status_and_layout::status_line_branch_state_resets_when_git_branch_disabled
        PASS [   0.146s] codex-tui chatwidget::tests::status_and_layout::status_line_context_remaining_renders_labeled_percent
        PASS [   0.138s] codex-tui chatwidget::tests::status_and_layout::status_line_context_used_renders_labeled_percent
        PASS [   0.127s] codex-tui chatwidget::tests::status_and_layout::status_line_fast_mode_renders_on_and_off
        PASS [   0.116s] codex-tui chatwidget::tests::status_and_layout::status_line_five_hour_item_omits_weekly_only_limit
        PASS [   0.130s] codex-tui chatwidget::tests::status_and_layout::status_line_git_summary_items_render_values
        PASS [   0.829s] codex-tui chatwidget::tests::status_and_layout::rate_limit_switch_prompt_popup_snapshot
        PASS [   0.113s] codex-tui chatwidget::tests::status_and_layout::status_line_legacy_context_usage_renders_context_used_percent
        PASS [   0.118s] codex-tui chatwidget::tests::status_and_layout::status_line_invalid_items_warn_once
        PASS [   0.116s] codex-tui chatwidget::tests::status_and_layout::status_line_legacy_limit_items_prefer_matching_windows
        PASS [   0.140s] codex-tui chatwidget::tests::status_and_layout::status_line_model_with_reasoning_includes_fast_for_fast_capable_models
        PASS [   0.811s] codex-tui chatwidget::tests::status_and_layout::renamed_thread_footer_title_snapshot
        PASS [   0.130s] codex-tui chatwidget::tests::status_and_layout::status_line_model_with_reasoning_updates_on_mode_switch_without_manual_refresh
        PASS [   0.168s] codex-tui chatwidget::tests::status_and_layout::status_line_reasoning_updates_on_mode_switch_without_manual_refresh
        PASS [   0.128s] codex-tui chatwidget::tests::status_and_layout::status_line_shows_secondary_non_weekly_when_primary_is_weekly
        PASS [   0.139s] codex-tui chatwidget::tests::status_and_layout::status_line_secondary_only_non_weekly_limit_omits_primary_limit_item
        PASS [   0.781s] codex-tui chatwidget::tests::status_and_layout::session_start_hook_events_render_snapshot
        PASS [   0.168s] codex-tui chatwidget::tests::status_and_layout::status_line_single_monthly_primary_omits_weekly_limit_item
        PASS [   0.144s] codex-tui chatwidget::tests::status_and_layout::status_line_uses_secondary_fallback_for_unsupported_window
        PASS [   1.091s] codex-tui chatwidget::tests::status_and_layout::running_hook_does_not_displace_active_exec_cell
        PASS [   0.883s] codex-tui chatwidget::tests::status_and_layout::status_line_fast_mode_footer_snapshot
        PASS [   0.812s] codex-tui chatwidget::tests::status_and_layout::status_line_goal_complete_elapsed_footer_snapshot
        PASS [   0.854s] codex-tui chatwidget::tests::status_and_layout::status_line_goal_active_token_budget_footer_snapshot
        PASS [   0.206s] codex-tui chatwidget::tests::status_and_layout::stream_error_restores_hidden_status_indicator
        PASS [   0.210s] codex-tui chatwidget::tests::status_and_layout::stream_error_updates_status_indicator
        PASS [   0.104s] codex-tui chatwidget::tests::status_and_layout::test_rate_limit_warnings_use_generic_fallback_labels
        PASS [   0.122s] codex-tui chatwidget::tests::status_and_layout::test_rate_limit_warnings_monthly
        PASS [   0.826s] codex-tui chatwidget::tests::status_and_layout::status_line_model_with_reasoning_plan_mode_footer_snapshot
        PASS [   0.894s] codex-tui chatwidget::tests::status_and_layout::status_line_model_with_reasoning_context_remaining_footer_snapshot
        PASS [   0.127s] codex-tui chatwidget::tests::status_and_layout::test_rate_limit_warnings_use_secondary_fallback_for_unsupported_window
        PASS [   0.142s] codex-tui chatwidget::tests::status_and_layout::terminal_title_model_updates_on_model_change_without_manual_refresh
        PASS [   0.150s] codex-tui chatwidget::tests::status_and_layout::streaming_final_answer_keeps_task_running_state
        PASS [   0.182s] codex-tui chatwidget::tests::status_and_layout::thread_goal_update_for_other_thread_is_ignored
        PASS [   0.110s] codex-tui chatwidget::tests::status_and_layout::token_count_none_resets_context_indicator
        PASS [   0.954s] codex-tui chatwidget::tests::status_and_layout::status_line_model_with_reasoning_fast_footer_snapshot
        PASS [   0.114s] codex-tui chatwidget::tests::status_and_layout::token_usage_update_uses_runtime_context_window
        PASS [   0.148s] codex-tui chatwidget::tests::status_and_layout::usage_limit_error_remaps_stale_member_credits_state_to_usage_limit_prompt
        PASS [   0.124s] codex-tui chatwidget::tests::status_and_layout::warning_event_adds_warning_history_cell
        PASS [   0.131s] codex-tui chatwidget::tests::status_and_layout::workspace_owner_nudge_default_no_dismisses_without_sending
        PASS [   0.172s] codex-tui chatwidget::tests::status_and_layout::workspace_owner_limit_states_do_not_prompt_for_owner_nudge
        PASS [   0.851s] codex-tui chatwidget::tests::status_and_layout::status_widget_active_snapshot
        PASS [   0.136s] codex-tui chatwidget::tests::status_and_layout::workspace_owner_nudge_reappears_after_dismissing_no
        PASS [   0.131s] codex-tui chatwidget::tests::status_command_tests::status_command_overlapping_refreshes_update_matching_cells_only
        PASS [   0.994s] codex-tui chatwidget::tests::status_and_layout::status_widget_and_approval_modal_snapshot
        PASS [   0.131s] codex-tui chatwidget::tests::status_command_tests::status_command_refresh_updates_cached_limits_for_future_status_outputs
        PASS [   0.131s] codex-tui chatwidget::tests::status_command_tests::status_command_renders_immediately_and_refreshes_rate_limits_for_chatgpt_auth
        PASS [   0.118s] codex-tui chatwidget::tests::status_command_tests::status_command_renders_immediately_without_rate_limit_refresh
        PASS [   0.120s] codex-tui chatwidget::tests::status_command_tests::status_command_renders_instruction_sources_from_thread_session
        PASS [   0.103s] codex-tui chatwidget::tests::status_surface_previews::missing_project_root_uses_different_status_and_title_preview_sources
        PASS [   0.116s] codex-tui chatwidget::tests::status_command_tests::status_command_uses_catalog_default_reasoning_when_config_empty
        PASS [   0.802s] codex-tui chatwidget::tests::status_and_layout::ui_snapshots_small_heights_idle
        PASS [   0.827s] codex-tui chatwidget::tests::status_and_layout::ui_snapshots_small_heights_task_running
        PASS [   0.844s] codex-tui chatwidget::tests::status_and_layout::user_prompt_submit_app_server_hook_notifications_render_snapshot
        PASS [   0.798s] codex-tui chatwidget::tests::status_and_layout::workspace_member_usage_limit_prompts_and_sends_usage_limit
        PASS [   0.872s] codex-tui chatwidget::tests::status_and_layout::workspace_member_credits_depleted_prompts_and_sends_credits
        PASS [   0.880s] codex-tui chatwidget::tests::status_and_layout::workspace_owner_limit_states_render_state_specific_messages
  TRY 1 FAIL [   0.971s] codex-tui chatwidget::tests::status_and_layout::workspace_owner_credits_nudge_completion_renders_feedback
  stdout ---

    running 1 test
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Snapshot Summary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Snapshot file: tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__workspace_owner_credits_nudge_completion_feedback.snap
    Snapshot: codex_tui__chatwidget__tests__workspace_owner_credits_nudge_completion_feedback
    Source: C:\CodexLab\codex\codex-rs:1310
    ───────────────────────────────────────────────────────────────────────────────
    Expression: rendered_cases.join("\n---\n")
    ───────────────────────────────────────────────────────────────────────────────
    -old snapshot
    +new results
    ────────────┬──────────────────────────────────────────────────────────────────
        1       │-• Workspace owner notified.
              1 │+ⓘ Workspace owner notified.
        2     2 │
        3     3 │ ---
        4       │-• Workspace owner was already notified recently.
              4 │+ⓘ Workspace owner was already notified recently.
        5     5 │
        6     6 │ ---
        7       │-• Could not notify your workspace owner. Please try again.
              7 │+ⓘ Could not notify your workspace owner. Please try again.
    ────────────┴──────────────────────────────────────────────────────────────────
    To update snapshots run `cargo insta review`
    Stopped on the first failure. Run `cargo insta test` to run all snapshots.
    test chatwidget::tests::status_and_layout::workspace_owner_credits_nudge_completion_renders_feedback ... FAILED

    failures:

    failures:
        chatwidget::tests::status_and_layout::workspace_owner_credits_nudge_completion_renders_feedback

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.90s

  stderr ---
    stored new snapshot C:\CodexLab\codex\codex-rs\tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__workspace_owner_credits_nudge_completion_feedback.snap.new

    thread 'chatwidget::tests::status_and_layout::workspace_owner_credits_nudge_completion_renders_feedback' (13540) panicked at C:\Users\JohnDell\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\insta-1.46.3\src\runtime.rs:719:13:
    snapshot assertion for 'codex_tui__chatwidget__tests__workspace_owner_credits_nudge_completion_feedback' failed in line 1310
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.103s] codex-tui chatwidget::tests::status_surface_previews::status_surface_preview_omits_unavailable_rate_limit_items
        PASS [   0.102s] codex-tui chatwidget::tests::status_surface_previews::terminal_title_preview_uses_title_truncation_for_live_values
  TRY 1 FAIL [   0.984s] codex-tui chatwidget::tests::status_and_layout::workspace_owner_usage_limit_nudge_completion_renders_feedback
  stdout ---

    running 1 test
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Snapshot Summary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Snapshot file: tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__workspace_owner_usage_limit_nudge_completion_feedback.snap
    Snapshot: codex_tui__chatwidget__tests__workspace_owner_usage_limit_nudge_completion_feedback
    Source: C:\CodexLab\codex\codex-rs:1346
    ───────────────────────────────────────────────────────────────────────────────
    Expression: rendered_cases.join("\n---\n")
    ───────────────────────────────────────────────────────────────────────────────
    -old snapshot
    +new results
    ────────────┬──────────────────────────────────────────────────────────────────
        1       │-• Limit increase requested.
              1 │+ⓘ Limit increase requested.
        2     2 │
        3     3 │ ---
        4       │-• A limit increase was already requested recently.
              4 │+ⓘ A limit increase was already requested recently.
        5     5 │
        6     6 │ ---
        7       │-• Could not request a limit increase. Please try again.
              7 │+ⓘ Could not request a limit increase. Please try again.
    ────────────┴──────────────────────────────────────────────────────────────────
    To update snapshots run `cargo insta review`
    Stopped on the first failure. Run `cargo insta test` to run all snapshots.
    test chatwidget::tests::status_and_layout::workspace_owner_usage_limit_nudge_completion_renders_feedback ... FAILED

    failures:

    failures:
        chatwidget::tests::status_and_layout::workspace_owner_usage_limit_nudge_completion_renders_feedback

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.88s

  stderr ---
    stored new snapshot C:\CodexLab\codex\codex-rs\tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__workspace_owner_usage_limit_nudge_completion_feedback.snap.new

    thread 'chatwidget::tests::status_and_layout::workspace_owner_usage_limit_nudge_completion_renders_feedback' (18960) panicked at C:\Users\JohnDell\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\insta-1.46.3\src\runtime.rs:719:13:
    snapshot assertion for 'codex_tui__chatwidget__tests__workspace_owner_usage_limit_nudge_completion_feedback' failed in line 1346
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.892s] codex-tui chatwidget::tests::status_surface_previews::status_line_setup_popup_hardcoded_only_snapshot
        PASS [   0.861s] codex-tui chatwidget::tests::status_surface_previews::status_line_setup_popup_live_only_snapshot
        PASS [   0.897s] codex-tui chatwidget::tests::status_surface_previews::status_line_setup_popup_mixed_snapshot
        PASS [   0.879s] codex-tui chatwidget::tests::status_surface_previews::status_line_setup_popup_rate_limits_snapshot
        PASS [   0.823s] codex-tui chatwidget::tests::status_surface_previews::status_surface_preview_lines_live_only_snapshot
        PASS [   0.894s] codex-tui chatwidget::tests::status_surface_previews::status_surface_preview_lines_hardcoded_only_snapshot
        PASS [   0.925s] codex-tui chatwidget::tests::status_surface_previews::status_surface_preview_lines_mixed_snapshot
        PASS [   0.888s] codex-tui chatwidget::tests::status_surface_previews::status_surface_preview_lines_rate_limits_snapshot
        PASS [   0.290s] codex-tui chatwidget::tests::status_surface_previews::thread_title_falls_back_to_thread_id_when_unnamed
        PASS [   0.300s] codex-tui chatwidget::tests::terminal_title::terminal_title_action_required_blinks_when_animations_are_enabled
        PASS [   0.772s] codex-tui chatwidget::tests::status_surface_previews::terminal_title_setup_popup_hardcoded_only_snapshot
        PASS [   0.105s] codex-tui chatwidget::transcript::tests::active_cell_revision_wraps
        PASS [   0.066s] codex-tui chatwidget::transcript::tests::copy_history_tracks_latest_visible_turn
  TRY 2 FAIL [   1.054s] codex-tui chatwidget::tests::status_and_layout::workspace_owner_credits_nudge_completion_renders_feedback
  stdout ---

    running 1 test
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Snapshot Summary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Snapshot file: tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__workspace_owner_credits_nudge_completion_feedback.snap
    Snapshot: codex_tui__chatwidget__tests__workspace_owner_credits_nudge_completion_feedback
    Source: C:\CodexLab\codex\codex-rs:1310
    ───────────────────────────────────────────────────────────────────────────────
    Expression: rendered_cases.join("\n---\n")
    ───────────────────────────────────────────────────────────────────────────────
    -old snapshot
    +new results
    ────────────┬──────────────────────────────────────────────────────────────────
        1       │-• Workspace owner notified.
              1 │+ⓘ Workspace owner notified.
        2     2 │
        3     3 │ ---
        4       │-• Workspace owner was already notified recently.
              4 │+ⓘ Workspace owner was already notified recently.
        5     5 │
        6     6 │ ---
        7       │-• Could not notify your workspace owner. Please try again.
              7 │+ⓘ Could not notify your workspace owner. Please try again.
    ────────────┴──────────────────────────────────────────────────────────────────
    To update snapshots run `cargo insta review`
    Stopped on the first failure. Run `cargo insta test` to run all snapshots.
    test chatwidget::tests::status_and_layout::workspace_owner_credits_nudge_completion_renders_feedback ... FAILED

    failures:

    failures:
        chatwidget::tests::status_and_layout::workspace_owner_credits_nudge_completion_renders_feedback

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 1.00s

  stderr ---
    stored new snapshot C:\CodexLab\codex\codex-rs\tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__workspace_owner_credits_nudge_completion_feedback.snap.new

    thread 'chatwidget::tests::status_and_layout::workspace_owner_credits_nudge_completion_renders_feedback' (18324) panicked at C:\Users\JohnDell\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\insta-1.46.3\src\runtime.rs:719:13:
    snapshot assertion for 'codex_tui__chatwidget__tests__workspace_owner_credits_nudge_completion_feedback' failed in line 1310
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.087s] codex-tui chatwidget::turn_lifecycle::tests::start_and_finish_update_running_state
        PASS [   0.069s] codex-tui clipboard_copy::tests::local_non_wsl_falls_back_to_osc52_when_native_fails
        PASS [   0.109s] codex-tui chatwidget::turn_lifecycle::tests::budget_limited_turn_ids_are_consumed
        PASS [   0.463s] codex-tui chatwidget::tests::terminal_title::terminal_title_action_required_respects_spinner_setting
        PASS [   0.075s] codex-tui clipboard_copy::tests::local_uses_native_clipboard_first
        PASS [   0.081s] codex-tui clipboard_copy::tests::local_reports_both_errors_when_native_and_osc52_fail
        PASS [   0.086s] codex-tui clipboard_copy::tests::local_tmux_fallback_prefers_tmux_when_native_fails
        PASS [   0.332s] codex-tui chatwidget::tests::terminal_title::terminal_title_activity_indicators_do_not_animate_when_animations_are_disabled
        PASS [   0.079s] codex-tui clipboard_copy::tests::local_wsl_falls_back_to_osc52_when_native_and_powershell_fail
        PASS [   0.461s] codex-tui chatwidget::tests::terminal_title::terminal_title_shows_action_required_while_exec_approval_is_pending
        PASS [   0.087s] codex-tui clipboard_copy::tests::local_wsl_native_failure_uses_powershell_and_skips_osc52_on_success
        PASS [   0.062s] codex-tui clipboard_copy::tests::local_wsl_reports_native_powershell_and_osc52_errors_when_all_fail
        PASS [   0.059s] codex-tui clipboard_copy::tests::osc52_encoding_roundtrips
  TRY 2 FAIL [   1.032s] codex-tui chatwidget::tests::status_and_layout::workspace_owner_usage_limit_nudge_completion_renders_feedback
  stdout ---

    running 1 test
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Snapshot Summary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Snapshot file: tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__workspace_owner_usage_limit_nudge_completion_feedback.snap
    Snapshot: codex_tui__chatwidget__tests__workspace_owner_usage_limit_nudge_completion_feedback
    Source: C:\CodexLab\codex\codex-rs:1346
    ───────────────────────────────────────────────────────────────────────────────
    Expression: rendered_cases.join("\n---\n")
    ───────────────────────────────────────────────────────────────────────────────
    -old snapshot
    +new results
    ────────────┬──────────────────────────────────────────────────────────────────
        1       │-• Limit increase requested.
              1 │+ⓘ Limit increase requested.
        2     2 │
        3     3 │ ---
        4       │-• A limit increase was already requested recently.
              4 │+ⓘ A limit increase was already requested recently.
        5     5 │
        6     6 │ ---
        7       │-• Could not request a limit increase. Please try again.
              7 │+ⓘ Could not request a limit increase. Please try again.
    ────────────┴──────────────────────────────────────────────────────────────────
    To update snapshots run `cargo insta review`
    Stopped on the first failure. Run `cargo insta test` to run all snapshots.
    test chatwidget::tests::status_and_layout::workspace_owner_usage_limit_nudge_completion_renders_feedback ... FAILED

    failures:

    failures:
        chatwidget::tests::status_and_layout::workspace_owner_usage_limit_nudge_completion_renders_feedback

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.98s

  stderr ---
    stored new snapshot C:\CodexLab\codex\codex-rs\tui\src/chatwidget/snapshots\codex_tui__chatwidget__tests__workspace_owner_usage_limit_nudge_completion_feedback.snap.new

    thread 'chatwidget::tests::status_and_layout::workspace_owner_usage_limit_nudge_completion_renders_feedback' (8876) panicked at C:\Users\JohnDell\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\insta-1.46.3\src\runtime.rs:719:13:
    snapshot assertion for 'codex_tui__chatwidget__tests__workspace_owner_usage_limit_nudge_completion_feedback' failed in line 1346
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.077s] codex-tui clipboard_copy::tests::osc52_wraps_tmux_passthrough
        PASS [   0.084s] codex-tui clipboard_copy::tests::osc52_rejects_payload_larger_than_limit
        PASS [   0.092s] codex-tui clipboard_copy::tests::ssh_inside_tmux_falls_back_to_osc52_when_tmux_copy_fails
        PASS [   0.090s] codex-tui clipboard_copy::tests::ssh_inside_tmux_prefers_tmux_clipboard
        PASS [   0.090s] codex-tui clipboard_copy::tests::ssh_inside_tmux_reports_tmux_and_osc52_errors_when_both_fail
        PASS [   0.062s] codex-tui clipboard_copy::tests::ssh_returns_osc52_error_and_skips_native
        PASS [   0.075s] codex-tui clipboard_copy::tests::ssh_uses_osc52_and_skips_native_on_success
        PASS [   0.078s] codex-tui clipboard_copy::tests::tmux_clipboard_copy_ready_accepts_forwarding_configuration
        PASS [   0.067s] codex-tui clipboard_copy::tests::tmux_clipboard_copy_ready_rejects_disabled_forwarding
        PASS [   0.068s] codex-tui clipboard_copy::tests::tmux_clipboard_copy_ready_rejects_missing_ms_capability
        PASS [   0.073s] codex-tui clipboard_paste::pasted_paths_tests::normalize_double_quoted_windows_path
        PASS [   0.075s] codex-tui clipboard_paste::pasted_paths_tests::normalize_file_url_windows
        PASS [   0.088s] codex-tui clipboard_copy::tests::write_osc52_to_writer_emits_sequence_verbatim
        PASS [   0.061s] codex-tui clipboard_paste::pasted_paths_tests::normalize_shell_escaped_single_path
        PASS [   0.072s] codex-tui clipboard_paste::pasted_paths_tests::normalize_multiple_tokens_returns_none
        PASS [   0.074s] codex-tui clipboard_paste::pasted_paths_tests::normalize_simple_quoted_path_fallback
        PASS [   0.075s] codex-tui clipboard_paste::pasted_paths_tests::normalize_single_quoted_unix_path
        PASS [   0.899s] codex-tui chatwidget::tests::status_surface_previews::terminal_title_setup_popup_live_only_snapshot
        PASS [   0.075s] codex-tui clipboard_paste::pasted_paths_tests::normalize_single_quoted_windows_path
        PASS [   0.046s] codex-tui clipboard_paste::pasted_paths_tests::normalize_unc_windows_path
        PASS [   0.055s] codex-tui clipboard_paste::pasted_paths_tests::pasted_image_format_png_jpeg_unknown
        PASS [   0.060s] codex-tui clipboard_paste::pasted_search_query_tests::collapses_whitespace
        PASS [   0.080s] codex-tui clipboard_paste::pasted_paths_tests::normalize_unquoted_windows_path_with_spaces
        PASS [   0.084s] codex-tui clipboard_paste::pasted_paths_tests::pasted_image_format_with_windows_style_paths
        PASS [   0.090s] codex-tui config_update::tests::app_scoped_key_path_quotes_dotted_app_ids
        PASS [   0.958s] codex-tui chatwidget::tests::status_surface_previews::terminal_title_setup_popup_mixed_snapshot
        PASS [   0.099s] codex-tui config_update::tests::format_config_error_preserves_server_validation_message
        PASS [   0.106s] codex-tui config_update::tests::trusted_project_edit_targets_project_trust_level
        PASS [   0.106s] codex-tui custom_terminal::tests::diff_buffers_clear_to_end_starts_after_wide_char
        PASS [   0.090s] codex-tui custom_terminal::tests::diff_buffers_does_not_emit_clear_to_end_for_full_width_row
        PASS [   0.095s] codex-tui custom_terminal::tests::reset_cursor_style_emits_default_user_shape
        PASS [   0.975s] codex-tui chatwidget::tests::status_surface_previews::terminal_title_setup_popup_rate_limits_snapshot
        PASS [   0.100s] codex-tui custom_terminal::tests::terminal_draw_applies_requested_cursor_style
        PASS [   0.069s] codex-tui cwd_prompt::tests::cwd_prompt_ctrl_c_exits_instead_of_selecting
        PASS [   0.079s] codex-tui cwd_prompt::tests::cwd_prompt_can_select_current
        PASS [   0.054s] codex-tui cwd_prompt::tests::cwd_prompt_selects_session_by_default
        PASS [   0.069s] codex-tui debug_config::tests::debug_config_output_formats_unix_socket_permissions
        PASS [   0.092s] codex-tui debug_config::tests::debug_config_output_lists_all_layers_including_disabled
        PASS [   0.095s] codex-tui debug_config::tests::debug_config_output_lists_approvals_reviewer_as_requirement
        PASS [   0.088s] codex-tui debug_config::tests::debug_config_output_lists_managed_hooks_requirement
        PASS [   0.103s] codex-tui debug_config::tests::debug_config_output_lists_requirement_sources
        PASS [   0.102s] codex-tui debug_config::tests::debug_config_output_lists_session_flag_key_value_pairs
        PASS [   0.105s] codex-tui debug_config::tests::debug_config_output_normalizes_empty_web_search_mode_list
        PASS [   0.104s] codex-tui debug_config::tests::debug_config_output_shows_enterprise_managed_layer_value
        PASS [   0.101s] codex-tui debug_config::tests::debug_config_output_shows_legacy_mdm_layer_value
        PASS [   0.104s] codex-tui debug_config::tests::session_all_proxy_url_uses_http_when_socks_disabled
        PASS [   0.106s] codex-tui debug_config::tests::session_all_proxy_url_uses_socks_when_enabled
        PASS [   0.084s] codex-tui diff_render::tests::ansi16_del_style_uses_foreground_only
        PASS [   0.121s] codex-tui diff_render::tests::ansi16_add_style_uses_foreground_only
        PASS [   0.102s] codex-tui diff_render::tests::ansi16_disables_line_and_gutter_backgrounds
        PASS [   0.092s] codex-tui diff_render::tests::ansi256_dark_theme_uses_distinct_add_and_delete_backgrounds
        PASS [   0.105s] codex-tui diff_render::tests::ansi16_sign_styles_use_foreground_only
        PASS [   0.111s] codex-tui diff_render::tests::detect_lang_for_common_paths
        PASS [   0.084s] codex-tui diff_render::tests::display_path_prefers_cwd_without_git_repo
        PASS [   0.055s] codex-tui diff_render::tests::explicit_force_override_keeps_ansi16_on_windows_terminal
        PASS [   0.075s] codex-tui diff_render::tests::explicit_force_override_keeps_ansi256_on_windows_terminal
        PASS [   0.108s] codex-tui diff_render::tests::fallback_wrapping_uses_display_width_for_tabs_and_wide_chars
        PASS [   0.112s] codex-tui diff_render::tests::light_theme_wrapped_lines_keep_number_gutter_contrast
        PASS [   0.061s] codex-tui diff_render::tests::non_windows_terminal_keeps_ansi16_diff_palette
        PASS [   0.061s] codex-tui diff_render::tests::light_truecolor_theme_uses_readable_gutter_and_line_backgrounds
        PASS [   0.309s] codex-tui diff_render::tests::add_diff_uses_path_extension_for_highlighting
        PASS [   0.090s] codex-tui diff_render::tests::non_wt_windows_terminal_keeps_unknown_color_level_conservative
        PASS [   0.077s] codex-tui diff_render::tests::theme_scope_backgrounds_override_truecolor_fallback_when_available
        PASS [   0.055s] codex-tui diff_render::tests::theme_scope_backgrounds_quantize_to_ansi256
        PASS [   0.293s] codex-tui diff_render::tests::delete_diff_uses_path_extension_for_highlighting
        PASS [   0.065s] codex-tui diff_render::tests::truecolor_dark_theme_uses_configured_backgrounds
        PASS [   0.293s] codex-tui diff_render::tests::rename_diff_uses_destination_extension_for_highlighting
        PASS [   0.716s] codex-tui cwd_prompt::tests::cwd_prompt_fork_snapshot
        PASS [   0.748s] codex-tui cwd_prompt::tests::cwd_prompt_snapshot
        PASS [   0.721s] codex-tui diff_render::tests::ui_snapshot_ansi16_insert_delete_no_background
        PASS [   0.915s] codex-tui diff_render::tests::ui_snapshot_apply_delete_block
        PASS [   0.946s] codex-tui diff_render::tests::ui_snapshot_apply_multiple_files_block
        PASS [   0.973s] codex-tui diff_render::tests::ui_snapshot_apply_update_block_line_numbers_three_digits_text
        PASS [   1.036s] codex-tui diff_render::tests::ui_snapshot_apply_add_block
        PASS [   1.048s] codex-tui diff_render::tests::ui_snapshot_apply_update_block_relativizes_path
        PASS [   1.073s] codex-tui diff_render::tests::ui_snapshot_apply_update_block
        PASS [   0.929s] codex-tui diff_render::tests::ui_snapshot_apply_update_block_wraps_long_lines_text
        PASS [   0.971s] codex-tui diff_render::tests::ui_snapshot_apply_update_block_wraps_long_lines
        PASS [   0.887s] codex-tui diff_render::tests::ui_snapshot_apply_update_with_rename_block
        PASS [   0.085s] codex-tui diff_render::tests::windows_terminal_promotes_ansi16_to_truecolor_for_diffs
        PASS [   0.081s] codex-tui diff_render::tests::wrap_styled_spans_flushes_at_span_boundary
        PASS [   0.087s] codex-tui diff_render::tests::wrap_styled_spans_preserves_styles
        PASS [   0.250s] codex-tui diff_render::tests::update_diff_preserves_multiline_highlight_state_within_hunk
        PASS [   0.052s] codex-tui diff_render::tests::wrap_styled_spans_single_line
        PASS [   0.062s] codex-tui diff_render::tests::wrap_styled_spans_splits_long_content
        PASS [   0.059s] codex-tui diff_render::tests::wrap_styled_spans_wraps_before_first_overflowing_char
        PASS [   0.090s] codex-tui diff_render::tests::wrap_styled_spans_tabs_have_visible_width
        PASS [   0.098s] codex-tui diff_render::tests::wt_session_promotes_ansi16_to_truecolor_for_diffs
        PASS [   0.068s] codex-tui diff_render::tests::wt_session_promotes_unknown_color_level_to_truecolor
        PASS [   0.072s] codex-tui exec_cell::render::tests::command_truncation_ellipsis_does_not_include_transcript_hint
        PASS [   1.003s] codex-tui diff_render::tests::ui_snapshot_diff_gallery_120x40
        PASS [   0.075s] codex-tui exec_cell::render::tests::exploring_display_does_not_split_long_url_like_search_query
        PASS [   0.260s] codex-tui exec_cell::render::tests::active_command_without_animations_is_stable
        PASS [   0.267s] codex-tui exec_cell::render::tests::command_display_does_not_split_long_url_token
        PASS [   0.749s] codex-tui diff_render::tests::ui_snapshot_wrap_behavior_insert
        PASS [   0.786s] codex-tui diff_render::tests::ui_snapshot_theme_scope_background_resolution
        PASS [   0.081s] codex-tui exec_cell::render::tests::output_lines_ellipsis_includes_transcript_hint
        PASS [   0.097s] codex-tui exec_cell::render::tests::truncate_lines_middle_does_not_truncate_blank_prefixed_output_lines
        PASS [   0.292s] codex-tui exec_cell::render::tests::desired_transcript_height_accounts_for_wrapped_url_like_rows
        PASS [   0.105s] codex-tui exec_cell::render::tests::truncate_lines_middle_keeps_omitted_count_in_line_units
        PASS [   0.067s] codex-tui exec_command::tests::split_command_string_preserves_non_roundtrippable_windows_commands
        PASS [   1.041s] codex-tui diff_render::tests::ui_snapshot_diff_gallery_80x24
        PASS [   0.062s] codex-tui exec_command::tests::test_escape_command
        PASS [   1.028s] codex-tui diff_render::tests::ui_snapshot_diff_gallery_94x35
        PASS [   0.075s] codex-tui exec_command::tests::split_command_string_round_trips_shell_wrappers
        PASS [   1.007s] codex-tui diff_render::tests::ui_snapshot_syntax_highlighted_insert_wraps
        PASS [   0.969s] codex-tui diff_render::tests::ui_snapshot_syntax_highlighted_insert_wraps_text
        PASS [   0.253s] codex-tui exec_cell::render::tests::output_display_does_not_split_long_url_like_token_without_scheme
        PASS [   0.054s] codex-tui exec_command::tests::test_strip_bash_lc_and_escape
        PASS [   0.044s] codex-tui external_agent_config_migration::tests::escape_skips_prompt
        PASS [   0.049s] codex-tui external_agent_config_migration::tests::proceed_action_is_skipped_when_no_items_are_selected
        PASS [   0.068s] codex-tui external_agent_config_migration::tests::numeric_shortcuts_choose_actions
        PASS [   0.071s] codex-tui external_agent_config_migration::tests::proceed_returns_selected_items
        PASS [   0.076s] codex-tui external_agent_config_migration::tests::proceed_requires_at_least_one_selected_item
        PASS [   0.082s] codex-tui external_agent_config_migration::tests::skip_forever_returns_skip_forever_outcome
        PASS [   0.093s] codex-tui external_agent_config_migration::tests::toggle_item_then_proceed_keeps_remaining_selection
        PASS [   0.084s] codex-tui external_agent_config_migration_startup::tests::external_agent_config_migration_success_message_mentions_plugins_when_present
        PASS [   0.104s] codex-tui external_agent_config_migration_startup::tests::external_agent_config_migration_prompt_requires_trust_nux_entry
        PASS [   0.090s] codex-tui external_agent_config_migration_startup::tests::external_agent_config_migration_success_message_omits_plugins_copy_when_absent
        PASS [   0.289s] codex-tui exec_cell::render::tests::user_shell_output_is_limited_by_screen_lines
        PASS [   0.086s] codex-tui external_editor::tests::resolve_editor_errors_when_unset
        PASS [   0.110s] codex-tui external_agent_config_migration_startup::tests::visible_external_agent_config_migration_items_omits_recently_prompted_scopes
        PASS [   0.118s] codex-tui external_agent_config_migration_startup::tests::external_config_migration_scope_cooldown_expires_after_five_days
        PASS [   0.077s] codex-tui get_git_diff::tests::get_git_diff_accepts_diff_exit_code_one
        PASS [   0.082s] codex-tui external_editor::tests::resolve_editor_prefers_visual
        PASS [   0.127s] codex-tui external_agent_config_migration_startup::tests::visible_external_agent_config_migration_items_omits_hidden_scopes
        PASS [   0.081s] codex-tui get_git_diff::tests::get_git_diff_disables_helpers_for_tracked_and_untracked_diffs
        PASS [   0.072s] codex-tui get_git_diff::tests::get_git_diff_rejects_unexpected_git_diff_status
        PASS [   0.059s] codex-tui get_git_diff::tests::get_git_diff_returns_not_git_for_non_git_cwd
        PASS [   0.055s] codex-tui git_action_directives::tests::last_created_branch_cwd_uses_the_last_matching_directive
        PASS [   0.060s] codex-tui git_action_directives::tests::hides_malformed_directives_without_materializing_rows
        PASS [   0.063s] codex-tui git_action_directives::tests::preserves_non_directive_and_malformed_code_comment_text
        PASS [   0.069s] codex-tui goal_display::tests::format_goal_elapsed_seconds_is_compact
        PASS [   0.086s] codex-tui git_action_directives::tests::strips_and_parses_git_action_directives
        PASS [   0.086s] codex-tui goal_display::tests::goal_usage_summary_formats_time_and_budgeted_tokens
        PASS [   0.102s] codex-tui history_cell::hook_cell::tests::completed_hook_multiline_context_preserves_display_and_raw_lines
        PASS [   0.097s] codex-tui history_cell::hook_cell::tests::completed_hook_multiline_warning_prefixes_first_line_only
        PASS [   0.087s] codex-tui history_cell::hook_cell::tests::completed_hook_with_warning_uses_default_bold_bullet
        PASS [   0.089s] codex-tui history_cell::hook_cell::tests::pending_hook_does_not_animate_transcript
        PASS [   0.075s] codex-tui history_cell::hook_cell::tests::visible_hook_does_not_animate_transcript_when_animations_disabled
        PASS [   0.094s] codex-tui history_cell::hook_cell::tests::visible_hook_animates_transcript_when_animations_enabled
        PASS [   0.076s] codex-tui history_cell::hook_cell::tests::visible_hook_without_animations_omits_spinner
        PASS [   0.050s] codex-tui history_cell::tests::agent_markdown_cell_narrow_width_shows_prefix_only
        PASS [   0.066s] codex-tui history_cell::tests::agent_markdown_cell_does_not_split_words_after_inline_markdown
        PASS [   0.083s] codex-tui history_cell::tests::agent_markdown_cell_renders_source_at_different_widths
        PASS [   0.089s] codex-tui history_cell::tests::agent_markdown_cell_survives_insert_history_rewrap
        PASS [   0.066s] codex-tui history_cell::tests::completed_mcp_tool_call_accepts_data_url_image_blocks
        PASS [   0.098s] codex-tui history_cell::tests::completed_mcp_tool_call_image_after_text_returns_extra_cell
        PASS [   0.064s] codex-tui history_cell::tests::completed_mcp_tool_call_skips_invalid_image_blocks
        PASS [   0.796s] codex-tui external_agent_config_migration::tests::prompt_snapshot
        PASS [   0.759s] codex-tui git_action_directives::tests::renders_code_comment_directives_as_markdown
        PASS [   0.819s] codex-tui history_cell::tests::active_mcp_tool_call_snapshot
        PASS [   0.794s] codex-tui history_cell::tests::coalesces_reads_across_multiple_calls
        PASS [   0.745s] codex-tui history_cell::tests::completed_mcp_tool_call_error_snapshot
        PASS [   0.841s] codex-tui history_cell::tests::coalesced_reads_dedupe_names
        PASS [   0.820s] codex-tui history_cell::tests::coalesces_sequential_reads_within_one_call
        PASS [   0.061s] codex-tui history_cell::tests::consolidation_walker_replaces_agent_message_cells
        PASS [   0.079s] codex-tui history_cell::tests::composite_cell_preserves_child_web_links
        PASS [   0.785s] codex-tui history_cell::tests::completed_mcp_tool_call_multiple_outputs_snapshot
        PASS [   0.063s] codex-tui history_cell::tests::deprecation_notice_renders_summary_with_details
        PASS [   0.066s] codex-tui history_cell::tests::empty_agent_message_cell_transcript
        PASS [   0.059s] codex-tui history_cell::tests::final_message_separator_hides_short_worked_label_and_includes_runtime_metrics
        PASS [   0.058s] codex-tui history_cell::tests::image_generation_call_renders_saved_path
        PASS [   0.079s] codex-tui history_cell::tests::final_message_separator_includes_worked_label_after_one_minute
        PASS [   0.072s] codex-tui history_cell::tests::mcp_inventory_loading_without_animations_is_stable
        PASS [   0.951s] codex-tui history_cell::tests::completed_mcp_tool_call_multiple_outputs_inline_snapshot
        PASS [   0.822s] codex-tui history_cell::tests::completed_mcp_tool_call_success_snapshot
        PASS [   0.817s] codex-tui history_cell::tests::completed_mcp_tool_call_wrapped_outputs_snapshot
        PASS [   0.788s] codex-tui history_cell::tests::cyber_policy_error_event_snapshot
        PASS [   0.810s] codex-tui history_cell::tests::cyber_policy_error_event_narrow_snapshot
        PASS [   0.763s] codex-tui history_cell::tests::error_event_oversized_input_snapshot
        PASS [   0.757s] codex-tui history_cell::tests::mcp_inventory_loading_snapshot
        PASS [   0.075s] codex-tui history_cell::tests::plan_update_does_not_split_url_like_tokens_in_note_or_step
        PASS [   0.774s] codex-tui history_cell::tests::mcp_tools_output_from_statuses_renders_verbose_inventory
        PASS [   0.077s] codex-tui history_cell::tests::prefixed_wrapped_history_cell_does_not_split_url_like_token
        PASS [   0.810s] codex-tui history_cell::tests::mcp_tools_output_from_statuses_renders_status_only_servers
        PASS [   0.049s] codex-tui history_cell::tests::prefixed_wrapped_history_cell_indents_wrapped_lines
        PASS [   0.090s] codex-tui history_cell::tests::prefixed_wrapped_history_cell_height_matches_wrapped_rendering
        PASS [   0.083s] codex-tui history_cell::tests::proposed_plan_cell_preserves_wrapped_table_web_links
        PASS [   0.843s] codex-tui history_cell::tests::mcp_tools_output_masks_sensitive_values
        PASS [   0.902s] codex-tui history_cell::tests::mcp_tools_output_lists_tools_for_hyphenated_server_names
        PASS [   0.090s] codex-tui history_cell::tests::proposed_plan_cell_renders_markdown_table
        PASS [   0.090s] codex-tui history_cell::tests::proposed_plan_cell_unwraps_markdown_fenced_table
        PASS [   1.020s] codex-tui history_cell::tests::multiline_command_both_lines_wrap_with_correct_prefixes
        PASS [   0.966s] codex-tui history_cell::tests::multiline_command_without_wrap_uses_branch_then_eight_spaces
        PASS [   0.741s] codex-tui history_cell::tests::plan_update_with_note_and_wrapping_snapshot
        PASS [   0.088s] codex-tui history_cell::tests::raw_lines_from_source_preserves_explicit_blank_lines
        PASS [   0.765s] codex-tui history_cell::tests::plan_update_without_note_snapshot
        PASS [   0.086s] codex-tui history_cell::tests::raw_lines_from_source_preserves_trailing_blank_but_not_trailing_newline
        PASS [   0.890s] codex-tui history_cell::tests::multiline_command_wraps_with_extra_indent_on_subsequent_lines
        PASS [   0.077s] codex-tui history_cell::tests::reasoning_summary_block
        PASS [   0.107s] codex-tui history_cell::tests::reasoning_summary_block_falls_back_when_header_is_missing
        PASS [   0.077s] codex-tui history_cell::tests::reasoning_summary_block_falls_back_when_summary_is_missing
        PASS [   0.089s] codex-tui history_cell::tests::reasoning_summary_block_returns_reasoning_cell_when_feature_disabled
        PASS [   0.787s] codex-tui history_cell::tests::ps_output_empty_snapshot
        PASS [   0.807s] codex-tui history_cell::tests::ps_output_chunk_leading_whitespace_snapshot
        PASS [   0.060s] codex-tui history_cell::tests::reasoning_summary_block_splits_header_and_summary_when_present
        PASS [   0.155s] codex-tui history_cell::tests::reasoning_summary_block_respects_config_overrides
        PASS [   0.059s] codex-tui history_cell::tests::reasoning_summary_height_matches_wrapped_rendering_for_url_like_content
        PASS [   0.782s] codex-tui history_cell::tests::ps_output_long_command_snapshot
        PASS [   0.772s] codex-tui history_cell::tests::ps_output_many_sessions_snapshot
        PASS [   0.060s] codex-tui history_cell::tests::render_clears_area_when_cell_content_shrinks
        PASS [   0.053s] codex-tui history_cell::tests::render_uses_wrapping_for_long_url_like_line
        PASS [   0.069s] codex-tui history_cell::tests::session_header_directory_center_truncates
        PASS [   0.816s] codex-tui history_cell::tests::ps_output_multiline_snapshot
        PASS [   0.067s] codex-tui history_cell::tests::session_header_directory_front_truncates_long_segment
        PASS [   0.071s] codex-tui history_cell::tests::session_header_hides_fast_status_when_disabled
        PASS [   0.078s] codex-tui history_cell::tests::session_info_first_event_suppresses_tooltips_and_nux
        PASS [   0.079s] codex-tui history_cell::tests::session_header_includes_reasoning_level_when_present
        PASS [   0.065s] codex-tui history_cell::tests::source_backed_cells_render_raw_source_without_prefix_or_style
        PASS [   0.113s] codex-tui history_cell::tests::session_info_uses_availability_nux_tooltip_override
        PASS [   0.132s] codex-tui history_cell::tests::session_info_hides_tooltips_when_disabled
        PASS [   0.074s] codex-tui history_cell::tests::structured_tool_cell_renders_raw_plain_text_without_prefix_or_style
        PASS [   0.078s] codex-tui history_cell::tests::unified_exec_interaction_cell_does_not_split_url_like_stdin_token
        PASS [   1.007s] codex-tui history_cell::tests::ran_cell_multiline_with_stderr_snapshot
        PASS [   0.072s] codex-tui history_cell::tests::unified_exec_interaction_cell_height_matches_wrapped_rendering
        PASS [   0.085s] codex-tui history_cell::tests::unified_exec_interaction_cell_renders_input
        PASS [   0.073s] codex-tui history_cell::tests::unified_exec_interaction_cell_renders_wait
        PASS [   0.062s] codex-tui history_cell::tests::user_history_cell_height_matches_rendered_lines_with_remote_images
        PASS [   0.926s] codex-tui history_cell::tests::raw_mode_toggle_transcript_snapshot
        PASS [   0.089s] codex-tui history_cell::tests::user_history_cell_summarizes_inline_data_urls
        PASS [   0.098s] codex-tui history_cell::tests::user_history_cell_trims_trailing_blank_message_lines
        PASS [   0.759s] codex-tui history_cell::tests::standalone_unix_update_available_history_cell_snapshot
        PASS [   0.848s] codex-tui history_cell::tests::standalone_windows_update_available_history_cell_snapshot
        PASS [   0.817s] codex-tui history_cell::tests::streamed_agent_list_paragraph_preserves_item_indent_when_wrapped
        PASS [   0.122s] codex-tui history_cell::tests::user_history_cell_trims_trailing_blank_message_lines_with_text_elements
        PASS [   0.836s] codex-tui history_cell::tests::streaming_agent_tail_blank_line_uses_one_viewport_row
        PASS [   0.071s] codex-tui history_cell::tests::web_search_history_cell_short_query_does_not_wrap
        PASS [   0.973s] codex-tui history_cell::tests::single_line_command_wraps_with_four_space_continuation
        PASS [   0.979s] codex-tui history_cell::tests::single_line_command_compact_when_fits
        PASS [   0.950s] codex-tui history_cell::tests::stderr_tail_more_than_five_lines_snapshot
        PASS [   0.091s] codex-tui history_cell::tests::yolo_mode_excludes_external_sandbox_profiles
        PASS [   0.099s] codex-tui history_cell::tests::web_search_history_cell_wraps_with_indented_continuation
        PASS [   0.100s] codex-tui history_cell::tests::wrapped_and_prefixed_cells_handle_tiny_widths
        PASS [   0.077s] codex-tui history_cell::tests::yolo_mode_includes_managed_full_access_profiles
        PASS [   0.050s] codex-tui ide_context::prompt::tests::apply_ide_context_uses_desktop_prompt_request_delimiter
        PASS [   0.063s] codex-tui ide_context::prompt::tests::extract_prompt_request_returns_text_after_last_delimiter
        PASS [   0.077s] codex-tui ide_context::prompt::tests::render_prompt_context_includes_selection_ranges_without_content
        PASS [   0.084s] codex-tui ide_context::prompt::tests::render_prompt_context_matches_app_format
        PASS [   0.763s] codex-tui history_cell::tests::user_history_cell_numbers_multiple_remote_images
        PASS [   0.080s] codex-tui ide_context::prompt::tests::render_prompt_context_omits_empty_context
        PASS [   0.085s] codex-tui ide_context::prompt::tests::render_prompt_context_omits_excess_open_tabs
        PASS [   0.073s] codex-tui ide_context::tests::deserializes_existing_ide_context_shape
        PASS [   0.068s] codex-tui insert_history::tests::vt100_blockquote_line_emits_green_fg
        PASS [   0.081s] codex-tui ide_context::prompt::tests::render_prompt_context_truncates_large_selection
        PASS [   0.777s] codex-tui history_cell::tests::user_history_cell_renders_remote_image_urls
        PASS [   0.050s] codex-tui insert_history::tests::vt100_blockquote_wrap_preserves_color_on_all_wrapped_lines
        PASS [   0.088s] codex-tui insert_history::tests::vt100_colored_prefix_then_plain_text_resets_color
        PASS [   0.078s] codex-tui insert_history::tests::vt100_long_unwrapped_url_does_not_insert_extra_blank_gap_before_content
        PASS [   0.085s] codex-tui insert_history::tests::vt100_prefixed_mixed_url_line_preserves_prefix_on_wrapped_rows
        PASS [   0.100s] codex-tui insert_history::tests::vt100_deep_nested_mixed_list_third_level_marker_is_colored
        PASS [   0.075s] codex-tui insert_history::tests::vt100_prefixed_non_url_line_preserves_prefix_on_wrapped_rows
        PASS [   0.109s] codex-tui insert_history::tests::vt100_prefixed_mixed_url_line_wraps_suffix_words_together
        PASS [   0.054s] codex-tui insert_history::tests::vt100_prefixed_url_like_without_scheme_keeps_prefix_and_token_on_same_row
        PASS [   0.065s] codex-tui insert_history::tests::vt100_prefixed_url_keeps_prefix_and_url_on_same_row
        PASS [   0.075s] codex-tui insert_history::tests::vt100_terminal_wrap_policy_does_not_pre_wrap_long_paragraph
        PASS [   0.059s] codex-tui insert_history::tests::vt100_unwrapped_url_like_clears_continuation_rows
        PASS [   0.082s] codex-tui insert_history::tests::writes_bold_then_regular_spans
        PASS [   0.065s] codex-tui key_hint::tests::ctrl_binding_does_not_match_ambiguous_c0_escape_or_delete
        PASS [   0.087s] codex-tui insert_history::tests::writes_semantic_web_link_without_changing_visible_text
        PASS [   0.089s] codex-tui key_hint::tests::ctrl_alt_sets_both_modifiers
        PASS [   0.055s] codex-tui key_hint::tests::ctrl_bindings_match_all_supported_c0_control_char_events
        PASS [   0.076s] codex-tui key_hint::tests::has_ctrl_or_alt_checks_supported_modifier_combinations
        PASS [   0.088s] codex-tui key_hint::tests::ctrl_letter_binding_matches_c0_control_char_events
        PASS [   0.073s] codex-tui key_hint::tests::history_search_ctrl_bindings_match_c0_control_char_events
        PASS [   0.064s] codex-tui key_hint::tests::is_press_accepts_press_and_repeat_but_rejects_release
        PASS [   0.805s] codex-tui history_cell::tests::user_history_cell_wraps_and_prefixes_each_line_snapshot
        PASS [   0.062s] codex-tui key_hint::tests::keybinding_list_ext_matches_any_binding
        PASS [   0.076s] codex-tui key_hint::tests::shift_letter_binding_does_not_match_plain_lowercase_or_other_uppercase
        PASS [   0.081s] codex-tui key_hint::tests::shift_letter_binding_preserves_other_modifiers_with_uppercase_compat
        PASS [   0.069s] codex-tui keymap::tests::configured_app_bindings_prune_new_list_default_overlaps
        PASS [   0.080s] codex-tui key_hint::tests::shifted_letter_binding_matches_uppercase_char_events
        PASS [   0.762s] codex-tui history_cell::tests::web_search_history_cell_snapshot
        PASS [   0.766s] codex-tui history_cell::tests::web_search_history_cell_transcript_snapshot
        PASS [   0.070s] codex-tui keymap::tests::configured_approval_bindings_prune_new_list_default_overlaps
        PASS [   0.758s] codex-tui history_cell::tests::web_search_history_cell_without_detail_snapshot
        PASS [   0.062s] codex-tui keymap::tests::configured_legacy_list_bindings_can_prune_all_new_default_keys
        PASS [   0.080s] codex-tui keymap::tests::configured_legacy_list_bindings_prune_new_default_overlaps
        PASS [   0.070s] codex-tui keymap::tests::configured_legacy_vim_normal_bindings_prune_new_change_operator_default
        PASS [   0.089s] codex-tui keymap::tests::configured_legacy_vim_normal_bindings_prune_new_substitute_default
        PASS [   0.109s] codex-tui keymap::tests::configured_legacy_vim_operator_bindings_prune_new_text_object_defaults
        PASS [   0.069s] codex-tui keymap::tests::deduplicates_repeated_bindings_while_preserving_first_seen_order
        PASS [   0.078s] codex-tui keymap::tests::default_approval_open_fullscreen_includes_ctrl_shift_a
        PASS [   0.092s] codex-tui keymap::tests::configured_main_surface_bindings_prune_reasoning_fallback_aliases
        PASS [   0.088s] codex-tui keymap::tests::default_composer_toggle_shortcuts_includes_shift_question_mark
        PASS [   0.083s] codex-tui keymap::tests::default_copy_binding_is_ctrl_o
        PASS [   0.055s] codex-tui keymap::tests::default_editor_deletion_includes_modified_backspace_delete_aliases
        PASS [   0.063s] codex-tui keymap::tests::defaults_include_list_page_and_jump_actions
        PASS [   0.076s] codex-tui keymap::tests::default_editor_insert_newline_includes_current_aliases
        PASS [   0.090s] codex-tui keymap::tests::default_editor_delete_forward_word_includes_alt_d
        PASS [   0.085s] codex-tui keymap::tests::defaults_include_reassignable_main_surface_actions
        PASS [   0.085s] codex-tui keymap::tests::defaults_pass_conflict_validation
        PASS [   0.093s] codex-tui keymap::tests::explicit_empty_array_unbinds_action
        PASS [   0.084s] codex-tui keymap::tests::explicit_new_list_bindings_still_conflict_with_configured_approval_bindings
        PASS [   0.088s] codex-tui keymap::tests::explicit_new_list_bindings_still_conflict_with_legacy_bindings
        PASS [   0.089s] codex-tui keymap::tests::explicit_new_vim_normal_binding_still_conflicts_with_legacy_binding
        PASS [   0.082s] codex-tui keymap::tests::explicit_new_vim_normal_substitute_binding_still_conflicts_with_legacy_binding
        PASS [   0.085s] codex-tui keymap::tests::explicit_new_vim_operator_binding_still_conflicts_with_legacy_binding
        PASS [   0.083s] codex-tui keymap::tests::explicit_reasoning_binding_still_conflicts_with_editor_binding
        PASS [   0.074s] codex-tui keymap::tests::falls_back_to_global_binding_when_context_override_is_not_set
        PASS [   0.085s] codex-tui keymap::tests::interrupt_turn_allows_backtrack_escape_and_can_be_remapped_or_unbound
        PASS [   0.078s] codex-tui keymap::tests::interrupt_turn_rejects_other_fixed_shortcuts
        PASS [   0.073s] codex-tui keymap::tests::interrupt_turn_rejects_request_user_input_question_navigation_bindings
        PASS [   0.083s] codex-tui keymap::tests::invalid_global_copy_binding_reports_global_path
        PASS [   0.076s] codex-tui keymap::tests::invalid_global_open_external_editor_binding_reports_global_path
        PASS [   0.077s] codex-tui keymap::tests::invalid_global_open_transcript_binding_reports_global_path
        PASS [   0.084s] codex-tui keymap::tests::kill_whole_line_can_be_assigned_without_default_binding
        PASS [   0.073s] codex-tui keymap::tests::kill_whole_line_conflicts_until_kill_line_start_is_unbound
        PASS [   0.071s] codex-tui keymap::tests::parses_all_named_non_character_keys
        PASS [   0.067s] codex-tui keymap::tests::parses_canonical_binding
        PASS [   0.068s] codex-tui keymap::tests::parses_function_keys_and_rejects_out_of_range_function_keys
        PASS [   0.048s] codex-tui keymap::tests::raw_output_toggle_can_be_remapped
        PASS [   0.078s] codex-tui keymap::tests::primary_binding_returns_first_or_none
        PASS [   0.058s] codex-tui keymap::tests::raw_output_toggle_defaults_to_alt_r
        PASS [   0.091s] codex-tui keymap::tests::parses_minus_alias_and_legacy_literal_minus
        PASS [   0.069s] codex-tui keymap::tests::reassignable_fixed_shortcuts_conflict_until_original_action_is_unbound
        PASS [   0.073s] codex-tui keymap::tests::rejects_conflicting_approval_bindings
        PASS [   0.779s] codex-tui insert_history::tests::vt100_zellij_raw_insert_keeps_soft_wrapped_tail_above_viewport
        PASS [   0.766s] codex-tui insert_history::tests::vt100_zellij_raw_replay_keeps_overflowing_soft_wrapped_tail_above_viewport
        PASS [   0.079s] codex-tui keymap::tests::rejects_conflicting_approval_deny_binding
        PASS [   0.077s] codex-tui keymap::tests::rejects_conflicting_approval_overlay_accept_binding
        PASS [   0.073s] codex-tui keymap::tests::rejects_conflicting_approval_overlay_cancel_binding
        PASS [   0.060s] codex-tui keymap::tests::rejects_conflicting_editor_bindings
        PASS [   0.063s] codex-tui keymap::tests::rejects_conflicting_list_bindings
        PASS [   0.073s] codex-tui keymap::tests::rejects_conflicting_list_page_and_jump_bindings
        PASS [   0.065s] codex-tui keymap::tests::rejects_conflicting_pager_bindings
        PASS [   0.069s] codex-tui keymap::tests::rejects_main_bindings_that_collide_with_remaining_fixed_shortcuts
        PASS [   0.076s] codex-tui keymap::tests::rejects_modifier_only_and_nonnumeric_function_key_specs
        PASS [   0.076s] codex-tui keymap::tests::rejects_pager_bindings_that_collide_with_transcript_backtrack_keys
        PASS [   0.075s] codex-tui keymap::tests::rejects_shadowing_approval_binding_in_app_scope
        PASS [   0.084s] codex-tui keymap::tests::rejects_shadowing_composer_binding_in_app_scope
        PASS [   0.079s] codex-tui keymap::tests::rejects_shadowing_composer_queue_in_app_scope
        PASS [   0.086s] codex-tui keymap::tests::rejects_shadowing_composer_toggle_shortcuts_in_app_scope
        PASS [   0.085s] codex-tui keymap::tests::rejects_shadowing_editor_binding_from_outer_main_handler
        PASS [   0.078s] codex-tui keymap::tests::rejects_shadowing_editor_binding_in_main_scope
        PASS [   0.084s] codex-tui keymap::tests::rejects_shadowing_list_binding_in_app_scope
        PASS [   0.078s] codex-tui keymap::tests::supports_string_or_array_bindings
        PASS [   0.079s] codex-tui keymap::tests::toggle_fast_mode_can_be_assigned_without_default_binding
        PASS [   0.084s] codex-tui keymap::tests::toggle_fast_mode_conflicts_with_existing_main_surface_bindings
        PASS [   0.079s] codex-tui keymap::tests::vim_normal_defaults_include_insert_and_arrow_aliases
        PASS [   0.082s] codex-tui keymap_setup::tests::action_menu_disables_clear_when_action_has_no_custom_binding
        PASS [   0.082s] codex-tui keymap_setup::tests::add_alternate_duplicate_is_noop
        PASS [   0.084s] codex-tui keymap_setup::tests::add_alternate_grows_default_multi_binding
        PASS [   0.085s] codex-tui keymap_setup::tests::add_alternate_grows_single_binding
        PASS [   0.098s] codex-tui keymap_setup::tests::capture_completion_returns_to_selected_keymap_picker_row
        PASS [   0.090s] codex-tui keymap_setup::tests::clear_removes_custom_binding
        PASS [   0.099s] codex-tui keymap_setup::tests::clear_completion_returns_to_selected_keymap_picker_row
        PASS [   0.097s] codex-tui keymap_setup::tests::debug_view_labels_custom_global_fallback_source
        PASS [   0.076s] codex-tui keymap_setup::tests::key_capture_serializes_c0_control_chars_as_ctrl_bindings
        PASS [   0.111s] codex-tui keymap_setup::tests::debug_view_uses_custom_binding_source
        PASS [   0.079s] codex-tui keymap_setup::tests::key_capture_serializes_function_keys_through_f24
        PASS [   0.093s] codex-tui keymap_setup::tests::key_capture_serializes_minus_as_named_key
        PASS [   0.075s] codex-tui keymap_setup::tests::key_capture_serializes_modifier_order_for_config
        PASS [   0.065s] codex-tui keymap_setup::tests::key_capture_serializes_special_keys
        PASS [   0.092s] codex-tui keymap_setup::tests::picker_approval_tab_lists_all_approval_actions
        PASS [   0.078s] codex-tui keymap_setup::tests::picker_common_tab_lists_curated_actions
        PASS [   0.141s] codex-tui keymap_setup::tests::picker_covers_every_replaceable_action
        PASS [   0.093s] codex-tui keymap_setup::tests::picker_customized_tab_contains_root_overrides
        PASS [   0.744s] codex-tui keymap_setup::tests::action_menu_content_snapshot
        PASS [   0.078s] codex-tui keymap_setup::tests::picker_debug_tab_is_last_and_opens_inspector
        PASS [   0.075s] codex-tui keymap_setup::tests::picker_hides_fast_mode_action_when_feature_is_disabled
        PASS [   0.814s] codex-tui keymap_setup::tests::capture_view_snapshot
        PASS [   0.823s] codex-tui keymap_setup::tests::debug_view_initial_snapshot
        PASS [   0.782s] codex-tui keymap_setup::tests::debug_view_shows_delayed_missing_key_hint
        PASS [   0.809s] codex-tui keymap_setup::tests::debug_view_reports_detected_key_and_matching_actions
        PASS [   0.090s] codex-tui keymap_setup::tests::picker_narrow_uses_compact_tabs
        PASS [   0.071s] codex-tui keymap_setup::tests::picker_selected_action_starts_on_matching_all_tab_row
        PASS [   0.729s] codex-tui keymap_setup::tests::picker_all_tab_items_remain_searchable
        PASS [   0.083s] codex-tui keymap_setup::tests::picker_unbound_tab_lists_default_unbound_actions
        PASS [   0.088s] codex-tui keymap_setup::tests::picker_shows_fast_mode_action_when_feature_is_enabled
        PASS [   0.080s] codex-tui keymap_setup::tests::replace_all_collapses_multi_binding_to_single
        PASS [   0.784s] codex-tui keymap_setup::tests::keymap_picker_fast_mode_enabled_snapshot
        PASS [   0.073s] codex-tui keymap_setup::tests::replace_one_completion_drops_focused_keymap_submenus
        PASS [   0.077s] codex-tui keymap_setup::tests::replace_one_deduplicates_replacement
        PASS [   0.084s] codex-tui keymap_setup::tests::replace_one_rejects_stale_old_key
        PASS [   0.074s] codex-tui keymap_setup::tests::replacement_rejects_unknown_action
        PASS [   0.093s] codex-tui keymap_setup::tests::replace_one_preserves_other_bindings
        PASS [   0.730s] codex-tui keymap_setup::tests::picker_content_snapshot
        PASS [   0.066s] codex-tui keymap_setup::tests::replacement_sets_single_binding
        PASS [   0.057s] codex-tui live_wrap::tests::fragmentation_invariance_long_token
        PASS [   0.047s] codex-tui live_wrap::tests::newline_splits_rows
        PASS [   0.058s] codex-tui live_wrap::tests::rewrap_on_width_change
        PASS [   0.068s] codex-tui live_wrap::tests::rows_do_not_exceed_width_ascii
        PASS [   0.080s] codex-tui live_wrap::tests::rows_do_not_exceed_width_emoji_cjk
        PASS [   0.116s] codex-tui local_chatgpt_auth::tests::loads_local_chatgpt_auth_from_managed_auth
        PASS [   0.097s] codex-tui local_chatgpt_auth::tests::preserves_usage_based_plan_type_wire_name
        PASS [   0.115s] codex-tui local_chatgpt_auth::tests::prefers_managed_auth_over_external_ephemeral_tokens
        PASS [   0.084s] codex-tui local_chatgpt_auth::tests::rejects_missing_local_auth
        PASS [   0.097s] codex-tui local_chatgpt_auth::tests::rejects_api_key_auth
        PASS [   0.068s] codex-tui markdown::tests::append_markdown_agent_keeps_markdown_fence_with_blank_line_between_header_and_delimiter
        PASS [   0.058s] codex-tui markdown::tests::append_markdown_agent_keeps_non_blockquoted_markdown_fence_with_blockquote_table_example
        PASS [   0.820s] codex-tui keymap_setup::tests::picker_custom_render_snapshot
        PASS [   0.053s] codex-tui markdown::tests::append_markdown_agent_unwraps_blockquoted_markdown_fence_table
        PASS [   0.097s] codex-tui markdown::tests::append_markdown_agent_unwraps_markdown_fences_for_no_outer_table_rendering
        PASS [   0.105s] codex-tui markdown::tests::append_markdown_agent_unwraps_markdown_fences_for_single_column_table
        PASS [   0.106s] codex-tui markdown::tests::append_markdown_agent_unwraps_markdown_fences_for_table_rendering
        PASS [   0.086s] codex-tui markdown::tests::append_markdown_agent_unwraps_markdown_fences_for_two_column_no_outer_table
        PASS [   0.079s] codex-tui markdown::tests::append_markdown_matches_tui_markdown_for_ordered_item
        PASS [   0.080s] codex-tui markdown::tests::append_markdown_keeps_ordered_list_line_unsplit_in_context
        PASS [  14.870s] codex-tui chatwidget::tests::status_and_layout::ambient_pet_stays_hidden_until_a_pet_is_selected
        PASS [   0.048s] codex-tui markdown::tests::citations_render_as_plain_text
        PASS [   0.068s] codex-tui markdown::tests::append_markdown_preserves_full_text_line
        PASS [   0.079s] codex-tui markdown::tests::indented_code_blocks_preserve_leading_whitespace
        PASS [   0.082s] codex-tui markdown::tests::unwrap_markdown_fences_repro_keeps_fence_without_header_delimiter_pair
        PASS [   0.276s] codex-tui markdown::tests::append_markdown_agent_keeps_markdown_fence_when_content_is_not_table
        PASS [   0.096s] codex-tui markdown_render::markdown_render_tests::blockquote_heading_inherits_heading_style
        PASS [   0.103s] codex-tui markdown_render::markdown_render_tests::blockquote_in_ordered_list_on_next_line
        PASS [   0.059s] codex-tui markdown_render::markdown_render_tests::blockquote_inside_nested_list
        PASS [   0.064s] codex-tui markdown_render::markdown_render_tests::blockquote_list_then_nested_blockquote
        PASS [   0.101s] codex-tui markdown_render::markdown_render_tests::blockquote_in_unordered_list_on_next_line
        PASS [   0.272s] codex-tui markdown::tests::append_markdown_agent_keeps_non_markdown_fences_as_code
        PASS [   0.061s] codex-tui markdown_render::markdown_render_tests::blockquote_multiple_with_break
        PASS [   0.086s] codex-tui markdown_render::markdown_render_tests::blockquote_single
        PASS [   0.089s] codex-tui markdown_render::markdown_render_tests::blockquote_nested_two_levels
        PASS [   0.077s] codex-tui markdown_render::markdown_render_tests::blockquote_soft_break
        PASS [   0.078s] codex-tui markdown_render::markdown_render_tests::blockquote_surrounded_by_blank_lines
        PASS [   0.063s] codex-tui markdown_render::markdown_render_tests::blockquote_with_list_items
        PASS [   0.077s] codex-tui markdown_render::markdown_render_tests::blockquote_two_paragraphs_inside_ordered_list_has_blank_line
        PASS [   0.078s] codex-tui markdown_render::markdown_render_tests::blockquote_with_code_block
        PASS [   0.077s] codex-tui markdown_render::markdown_render_tests::blockquote_with_heading_and_paragraph
        PASS [   0.093s] codex-tui markdown_render::markdown_render_tests::blockquote_three_paragraphs_short_lines
        PASS [   0.050s] codex-tui markdown_render::markdown_render_tests::blockquote_with_ordered_list
        PASS [   0.056s] codex-tui markdown_render::markdown_render_tests::blockquote_with_multiline_code_block
        PASS [   0.067s] codex-tui markdown_render::markdown_render_tests::code_block_indented
        PASS [   0.074s] codex-tui markdown_render::markdown_render_tests::code_block_inside_unordered_list_item_is_indented
        PASS [   0.074s] codex-tui markdown_render::markdown_render_tests::code_block_inside_unordered_list_item_multiple_lines
        PASS [   0.804s] codex-tui keymap_setup::tests::picker_narrow_render_snapshot
        PASS [   0.089s] codex-tui markdown_render::markdown_render_tests::code_block_multiple_lines_inside_unordered_list
        PASS [   0.086s] codex-tui markdown_render::markdown_render_tests::code_block_multiple_lines_root
        PASS [   0.089s] codex-tui markdown_render::markdown_render_tests::code_block_no_lang_plain
        PASS [   0.774s] codex-tui keymap_setup::tests::picker_wide_render_snapshot
        PASS [   0.083s] codex-tui markdown_render::markdown_render_tests::consecutive_unordered_list_local_file_links_do_not_detach_paths
        PASS [   0.082s] codex-tui markdown_render::markdown_render_tests::deeply_nested_mixed_three_levels
        PASS [   0.092s] codex-tui markdown_render::markdown_render_tests::emphasis
        PASS [   0.090s] codex-tui markdown_render::markdown_render_tests::empty
        PASS [   0.097s] codex-tui markdown_render::markdown_render_tests::file_link_appends_hash_anchor_when_label_lacks_it
        PASS [   0.124s] codex-tui markdown_render::markdown_render_tests::escaped_pipes_render_in_table_cells
        PASS [   0.087s] codex-tui markdown_render::markdown_render_tests::file_link_appends_hash_range_when_label_lacks_it
        PASS [   0.063s] codex-tui markdown_render::markdown_render_tests::file_link_appends_range_when_label_lacks_it
        PASS [   0.084s] codex-tui markdown_render::markdown_render_tests::file_link_appends_line_number_when_label_lacks_it
        PASS [   0.066s] codex-tui markdown_render::markdown_render_tests::file_link_decodes_percent_encoded_bare_path_destination
        PASS [   0.073s] codex-tui markdown_render::markdown_render_tests::file_link_hides_destination
        PASS [   0.256s] codex-tui markdown_render::markdown_render_tests::code_block_known_lang_has_syntax_colors
        PASS [   0.058s] codex-tui markdown_render::markdown_render_tests::file_link_keeps_absolute_paths_outside_cwd
        PASS [   0.241s] codex-tui markdown_render::markdown_render_tests::code_block_unknown_lang_plain
        PASS [   0.048s] codex-tui markdown_render::markdown_render_tests::file_link_uses_target_path_for_hash_anchor
        PASS [   0.055s] codex-tui markdown_render::markdown_render_tests::file_link_uses_target_path_for_hash_range
        PASS [   0.240s] codex-tui markdown_render::markdown_render_tests::code_block_with_inner_triple_backticks_outer_four
        PASS [   0.263s] codex-tui markdown_render::markdown_render_tests::code_block_preserves_trailing_blank_lines
        PASS [   0.067s] codex-tui markdown_render::markdown_render_tests::file_link_uses_target_path_for_range
        PASS [   0.073s] codex-tui markdown_render::markdown_render_tests::headings
        PASS [   0.081s] codex-tui markdown_render::markdown_render_tests::horizontal_rule_renders_em_dashes
        PASS [   0.074s] codex-tui markdown_render::markdown_render_tests::html_block_is_verbatim_multiline
        PASS [   0.082s] codex-tui markdown_render::markdown_render_tests::html_continuation_paragraph_in_unordered_item_indented
        PASS [   0.081s] codex-tui markdown_render::markdown_render_tests::html_in_tight_ordered_item_soft_breaks_with_space
        PASS [   0.073s] codex-tui markdown_render::markdown_render_tests::html_inline_is_verbatim
        PASS [   0.077s] codex-tui markdown_render::markdown_render_tests::inline_code
        PASS [   0.085s] codex-tui markdown_render::markdown_render_tests::link
        PASS [   0.093s] codex-tui markdown_render::markdown_render_tests::list_item_after_simple_item_stays_compact
        PASS [   0.101s] codex-tui markdown_render::markdown_render_tests::list_item_blockquote_then_text
        PASS [   0.097s] codex-tui markdown_render::markdown_render_tests::list_item_text_blockquote_text
        PASS [   0.097s] codex-tui markdown_render::markdown_render_tests::list_item_text_then_blockquote
        PASS [   0.105s] codex-tui markdown_render::markdown_render_tests::list_item_with_inline_blockquote_on_same_line
        PASS [   0.109s] codex-tui markdown_render::markdown_render_tests::list_nested
        PASS [   0.099s] codex-tui markdown_render::markdown_render_tests::list_ordered
        PASS [   0.101s] codex-tui markdown_render::markdown_render_tests::list_ordered_custom_start
        PASS [   0.114s] codex-tui markdown_render::markdown_render_tests::list_unordered_multiple
        PASS [   0.106s] codex-tui markdown_render::markdown_render_tests::list_unordered_single
        PASS [   0.101s] codex-tui markdown_render::markdown_render_tests::load_location_suffix_regexes
        PASS [   0.087s] codex-tui markdown_render::markdown_render_tests::loose_items_due_to_blank_line_between_items
        PASS [   0.091s] codex-tui markdown_render::markdown_render_tests::loose_list_item_multiple_paragraphs
        PASS [   0.080s] codex-tui markdown_render::markdown_render_tests::mixed_tight_then_loose_in_one_list
        PASS [   0.082s] codex-tui markdown_render::markdown_render_tests::multiline_file_link_label_after_styled_prefix_does_not_panic
        PASS [   0.082s] codex-tui markdown_render::markdown_render_tests::nested_blockquote_with_inline_and_fenced_code
        PASS [   0.081s] codex-tui markdown_render::markdown_render_tests::nested_item_continuation_paragraph_is_indented
        PASS [   0.091s] codex-tui markdown_render::markdown_render_tests::nested_five_levels_mixed_lists
        PASS [   0.088s] codex-tui markdown_render::markdown_render_tests::nested_unordered_in_ordered
        PASS [   0.112s] codex-tui markdown_render::markdown_render_tests::nested_ordered_in_unordered
        PASS [   0.086s] codex-tui markdown_render::markdown_render_tests::ordered_item_continuation_paragraph_is_indented
        PASS [   0.059s] codex-tui markdown_render::markdown_render_tests::ordered_item_with_indented_continuation_is_tight
        PASS [   0.079s] codex-tui markdown_render::markdown_render_tests::ordered_item_with_code_block_and_nested_bullet
        PASS [   0.052s] codex-tui markdown_render::markdown_render_tests::paragraph_multiple
        PASS [   0.074s] codex-tui markdown_render::markdown_render_tests::paragraph_single
        PASS [   0.064s] codex-tui markdown_render::markdown_render_tests::paragraph_soft_break
        PASS [   0.067s] codex-tui markdown_render::markdown_render_tests::strikethrough
        PASS [   0.049s] codex-tui markdown_render::markdown_render_tests::strong_emphasis
        PASS [   0.082s] codex-tui markdown_render::markdown_render_tests::strong
        PASS [   0.078s] codex-tui markdown_render::markdown_render_tests::table_alignment_respects_markers
        PASS [   0.089s] codex-tui markdown_render::markdown_render_tests::table_inside_blockquote_has_quote_prefix
        PASS [   0.093s] codex-tui markdown_render::markdown_render_tests::table_falls_back_to_key_value_records_if_grid_cannot_fit
        PASS [   0.243s] codex-tui markdown_render::markdown_render_tests::outer_list_item_after_nested_code_block_keeps_blank_separator
        PASS [   0.099s] codex-tui markdown_render::markdown_render_tests::table_key_value_fallback_preserves_rich_values_and_themed_labels
        PASS [   0.091s] codex-tui markdown_render::markdown_render_tests::table_renders_app_style_rows_with_themed_bold_header
        PASS [   0.111s] codex-tui markdown_render::markdown_render_tests::table_separates_logical_rows_after_wrapped_content
        PASS [   0.112s] codex-tui markdown_render::markdown_render_tests::tight_item_with_soft_break
        PASS [   0.076s] codex-tui markdown_render::markdown_render_tests::unordered_item_continuation_paragraph_is_indented
        PASS [   0.755s] codex-tui markdown_render::markdown_render_tests::markdown_render_file_link_snapshot
        PASS [   0.067s] codex-tui markdown_render::markdown_render_tests::unordered_list_local_file_link_soft_break_before_colon_stays_inline
        PASS [   0.062s] codex-tui markdown_render::markdown_render_tests::unordered_list_local_file_link_stays_inline_with_following_text
        PASS [   0.062s] codex-tui markdown_render::markdown_render_tests::url_link_shows_destination
        PASS [   0.848s] codex-tui markdown_render::markdown_render_tests::mixed_url_markdown_wraps_prose_without_splitting_words_snapshot
        PASS [   1.032s] codex-tui markdown_render::markdown_render_tests::list_item_after_code_block_keeps_blank_separator
        PASS [   0.879s] codex-tui markdown_render::markdown_render_tests::multiline_finding_items_are_separated_snapshot
        PASS [   0.079s] codex-tui markdown_render::markdown_render_tests::wrapped_list_item_is_separated_from_next_sibling
        PASS [   0.951s] codex-tui markdown_render::markdown_render_tests::markdown_render_complex_snapshot
        PASS [   0.074s] codex-tui markdown_render::tests::column_classification_compact_all_short
        PASS [   0.079s] codex-tui markdown_render::tests::annotates_explicit_web_link_label_and_visible_destination
        PASS [   0.058s] codex-tui markdown_render::tests::column_classification_narrative_by_word_count
        PASS [   0.055s] codex-tui markdown_render::tests::column_classification_token_heavy_by_url_like_tokens
        PASS [   0.080s] codex-tui markdown_render::tests::column_classification_token_heavy_for_local_path_lists
        PASS [   0.091s] codex-tui markdown_render::tests::does_not_split_long_url_like_token_without_scheme
        PASS [   0.083s] codex-tui markdown_render::tests::does_not_wrap_code_blocks
        PASS [   0.808s] codex-tui markdown_render::markdown_render_tests::table_keeps_grid_when_only_one_compact_record_fragments_snapshot
        PASS [   0.075s] codex-tui markdown_render::tests::next_column_to_shrink_prefers_token_heavy_then_narrative
        PASS [   0.088s] codex-tui markdown_render::tests::key_value_table_keeps_web_annotations
        PASS [   0.082s] codex-tui markdown_render::tests::pipe_table_fallback_keeps_web_annotations
        PASS [   0.065s] codex-tui markdown_render::tests::preferred_floor_compact_uses_body_token
        PASS [   0.780s] codex-tui markdown_render::markdown_render_tests::table_renders_records_when_multiple_prose_columns_are_starved_snapshot
        PASS [   0.065s] codex-tui markdown_render::tests::preferred_floor_narrative_retains_readable_width
        PASS [   0.251s] codex-tui markdown_render::tests::does_not_annotate_code_or_non_web_markdown_links
        PASS [   0.072s] codex-tui markdown_render::tests::preferred_floor_token_heavy_retains_readable_width
        PASS [   0.059s] codex-tui markdown_render::tests::spillover_detects_html_content
        PASS [   0.271s] codex-tui markdown_render::tests::crlf_code_block_no_extra_blank_lines
        PASS [   0.770s] codex-tui markdown_render::markdown_render_tests::table_wraps_file_paths_before_collapsing_narrative_columns_snapshot
        PASS [   0.054s] codex-tui markdown_render::tests::spillover_detects_label_followed_by_html
        PASS [   0.828s] codex-tui markdown_render::markdown_render_tests::table_renders_stacked_key_value_records_when_path_column_becomes_too_narrow_snapshot
        PASS [   0.066s] codex-tui markdown_render::tests::spillover_detects_single_cell_row
        PASS [   0.059s] codex-tui markdown_render::tests::spillover_detects_trailing_html_label
        PASS [   0.050s] codex-tui markdown_render::tests::spillover_keeps_label_when_next_is_not_html
        PASS [   0.259s] codex-tui markdown_render::tests::fenced_code_info_string_with_metadata_highlights
        PASS [   0.872s] codex-tui markdown_render::markdown_render_tests::table_renders_key_value_records_when_compact_fragmentation_is_systemic_snapshot
        PASS [   0.061s] codex-tui markdown_render::tests::spillover_keeps_normal_multi_cell_row
        PASS [   0.062s] codex-tui markdown_render::tests::spillover_keeps_single_cell_row_with_table_pipe_syntax
        PASS [   0.070s] codex-tui markdown_render::tests::wrap_cell_preserves_hard_break_lines
        PASS [   0.068s] codex-tui markdown_render::tests::wraps_blockquotes
        PASS [   0.080s] codex-tui markdown_render::tests::wrapped_table_url_fragments_keep_complete_web_destination
        PASS [   0.070s] codex-tui markdown_render::tests::wraps_blockquotes_inside_lists
        PASS [   0.070s] codex-tui markdown_render::tests::wraps_list_items_containing_blockquotes
        PASS [   0.068s] codex-tui markdown_render::tests::wraps_list_items_preserving_indent
        PASS [   0.067s] codex-tui markdown_render::tests::wraps_nested_lists
        PASS [   0.070s] codex-tui markdown_render::tests::wraps_ordered_lists
        PASS [   0.078s] codex-tui markdown_render::tests::wraps_plain_text_when_width_provided
        PASS [   0.082s] codex-tui markdown_stream::tests::e2e_stream_blockquote_nested_is_green
        PASS [   0.087s] codex-tui markdown_stream::tests::collector_source_chunks_round_trip_into_agent_fence_unwrapping
        PASS [   0.076s] codex-tui markdown_stream::tests::e2e_stream_blockquote_simple_is_green
        PASS [   0.077s] codex-tui markdown_stream::tests::e2e_stream_blockquote_with_list_items_is_green
        PASS [   0.078s] codex-tui markdown_stream::tests::e2e_stream_blockquote_wrap_preserves_green_style
        PASS [   0.084s] codex-tui markdown_stream::tests::e2e_stream_deep_nested_third_level_marker_is_light_blue
        PASS [   0.085s] codex-tui markdown_stream::tests::e2e_stream_nested_mixed_lists_ordered_marker_is_light_blue
        PASS [   0.084s] codex-tui markdown_stream::tests::empty_fenced_block_is_dropped_and_separator_preserved_before_heading
        PASS [   0.087s] codex-tui markdown_stream::tests::finalize_commits_partial_line
        PASS [   0.090s] codex-tui markdown_stream::tests::fuzz_class_bullet_duplication_variant_1
        PASS [   0.080s] codex-tui markdown_stream::tests::fuzz_class_bullet_duplication_variant_2
        PASS [   0.080s] codex-tui markdown_stream::tests::heading_not_inlined_when_split_across_chunks
        PASS [   0.087s] codex-tui markdown_stream::tests::heading_starts_on_new_line_when_following_paragraph
        PASS [   0.094s] codex-tui markdown_stream::tests::lists_and_fences_commit_without_duplication
        PASS [   0.087s] codex-tui markdown_stream::tests::loose_list_with_split_dashes_matches_full_render
        PASS [   0.040s] codex-tui markdown_stream::tests::table_like_lines_inside_fenced_code_are_not_held
        PASS [   0.094s] codex-tui markdown_stream::tests::no_commit_until_newline
        PASS [   0.101s] codex-tui markdown_stream::tests::paragraph_then_empty_fence_then_heading_keeps_heading_on_new_line
        PASS [   0.107s] codex-tui markdown_stream::tests::pipe_text_without_table_prefix_is_not_delayed
        PASS [   0.102s] codex-tui markdown_stream::tests::streaming_html_block_then_text_matches_full
        PASS [   0.159s] codex-tui markdown_stream::tests::loose_vs_tight_list_items_streaming_matches_full
        PASS [   0.133s] codex-tui markdown_stream::tests::table_header_commits_without_holdback
        PASS [   0.119s] codex-tui markdown_stream::tests::utf8_boundary_safety_and_wide_chars
        PASS [   0.123s] codex-tui mention_codec::tests::decode_history_mentions_restores_at_sigil_for_tool_paths
        PASS [   0.112s] codex-tui mention_codec::tests::decode_history_mentions_restores_plugin_links_with_at_sigil
        PASS [   0.122s] codex-tui mention_codec::tests::decode_history_mentions_restores_visible_tokens
        PASS [   0.113s] codex-tui mention_codec::tests::decode_history_mentions_without_at_mentions_ignores_at_non_plugin_paths
        PASS [   0.089s] codex-tui mention_codec::tests::encode_history_mentions_does_not_let_at_token_steal_later_tool_binding
        PASS [   0.110s] codex-tui mention_codec::tests::decode_history_mentions_without_at_mentions_uses_legacy_plugin_fallback
        PASS [   0.091s] codex-tui mention_codec::tests::encode_history_mentions_links_at_mentions_after_unicode_whitespace
        PASS [   0.085s] codex-tui mention_codec::tests::encode_history_mentions_links_both_sigils_for_same_name
        PASS [   0.057s] codex-tui mention_codec::tests::encode_history_mentions_links_bound_mentions_in_order
        PASS [   0.061s] codex-tui mention_codec::tests::encode_history_mentions_links_dollar_mentions_with_path_like_suffixes
        PASS [   0.069s] codex-tui mention_codec::tests::encode_history_mentions_links_dollar_mentions_after_punctuation
        PASS [   0.068s] codex-tui mention_codec::tests::encode_history_mentions_links_sentence_ending_at_mentions
        PASS [   0.071s] codex-tui mention_codec::tests::encode_history_mentions_links_parenthesized_at_mentions
        PASS [   0.071s] codex-tui mention_codec::tests::encode_history_mentions_preserves_at_sigils
        PASS [   0.075s] codex-tui mention_codec::tests::encode_history_mentions_skips_embedded_at_substrings
        PASS [   0.082s] codex-tui model_migration::tests::escape_key_accepts_prompt
        PASS [   0.088s] codex-tui model_migration::tests::markdown_prompt_keeps_long_url_tail_visible_when_narrow
        PASS [   0.095s] codex-tui model_migration::tests::selecting_use_existing_model_rejects_upgrade
        PASS [   0.098s] codex-tui motion::tests::reduced_motion_activity_indicator_uses_explicit_fallback
        PASS [   0.101s] codex-tui motion::tests::reduced_motion_shimmer_text_is_plain_text
        PASS [   0.105s] codex-tui multi_agents::tests::agent_shortcut_matches_option_arrows_only
        PASS [   0.067s] codex-tui multi_agents::tests::title_styles_nickname_and_role
        PASS [   0.055s] codex-tui notifications::osc9::tests::post_notification_writes_plain_osc9_sequence
        PASS [   0.089s] codex-tui notifications::osc9::tests::post_notification_escapes_escape_bytes_inside_tmux_payload
        PASS [   0.077s] codex-tui notifications::osc9::tests::post_notification_writes_tmux_dcs_wrapped_osc9_sequence
        PASS [   0.062s] codex-tui notifications::tests::selects_osc9_method
        PASS [   0.053s] codex-tui notifications::tests::supports_osc9_for_supported_terminals
        PASS [   0.087s] codex-tui notifications::tests::selects_bel_method
        PASS [   0.053s] codex-tui notifications::tests::supports_osc9_for_unsupported_terminals
        PASS [   0.064s] codex-tui npm_registry::tests::ready_version_rejects_missing_root_dist
        PASS [   0.071s] codex-tui npm_registry::tests::ready_version_rejects_stale_latest_dist_tag
        PASS [   0.086s] codex-tui npm_registry::tests::ready_version_requires_latest_dist_tag_and_root_dist
        PASS [   0.084s] codex-tui onboarding::auth::headless_chatgpt_login::tests::device_code_attempt_matches_only_for_matching_request_id
        PASS [   0.065s] codex-tui onboarding::auth::headless_chatgpt_login::tests::set_device_code_error_for_active_attempt_updates_only_when_active
        PASS [   0.061s] codex-tui onboarding::auth::headless_chatgpt_login::tests::set_device_code_state_for_active_attempt_updates_only_when_active
        LEAK [   0.379s] codex-tui onboarding::auth::tests::api_key_flow_disabled_when_chatgpt_forced
        PASS [   0.815s] codex-tui model_migration::tests::prompt_snapshot_gpt5_codex
        LEAK [   0.434s] codex-tui onboarding::auth::tests::auth_widget_suppresses_animations_when_device_code_is_visible
        PASS [   0.819s] codex-tui model_migration::tests::prompt_snapshot_gpt5_codex_mini
        LEAK [   0.452s] codex-tui onboarding::auth::tests::auth_widget_suppresses_animations_while_requesting_device_code
        PASS [   0.886s] codex-tui model_migration::tests::prompt_snapshot
        PASS [   0.899s] codex-tui model_migration::tests::prompt_snapshot_gpt5_family
        LEAK [   0.511s] codex-tui onboarding::auth::tests::cancel_active_attempt_notifies_device_code_login
        PASS [   0.066s] codex-tui onboarding::auth::tests::mark_url_hyperlink_sanitizes_control_chars
        PASS [   0.887s] codex-tui multi_agents::tests::collab_events_snapshot
        PASS [   0.083s] codex-tui onboarding::auth::tests::mark_url_hyperlink_wraps_cyan_underlined_cells
        PASS [   0.902s] codex-tui multi_agents::tests::collab_resume_interrupted_snapshot
        PASS [   0.071s] codex-tui onboarding::onboarding_screen::tests::does_not_suppress_control_quit_key_during_api_key_entry
        PASS [   0.078s] codex-tui onboarding::onboarding_screen::tests::does_not_suppress_when_not_in_api_key_entry
        PASS [   0.081s] codex-tui onboarding::onboarding_screen::tests::does_not_suppress_printable_quit_key_when_api_key_input_is_empty
        PASS [   0.095s] codex-tui onboarding::onboarding_screen::tests::suppresses_printable_quit_key_during_api_key_entry
        LEAK [   0.357s] codex-tui onboarding::auth::tests::cancel_active_attempt_resets_browser_login_state
        PASS [   0.112s] codex-tui onboarding::onboarding_screen::tests::trust_persistence_failure_keeps_trust_step_in_progress
        PASS [   0.040s] codex-tui onboarding::trust_directory::tests::release_event_does_not_change_selection
        LEAK [   0.379s] codex-tui onboarding::auth::tests::continue_in_browser_renders_osc8_hyperlink
        PASS [   0.060s] codex-tui onboarding::welcome::tests::ctrl_shift_dot_changes_animation_variant
        LEAK [   0.394s] codex-tui onboarding::auth::tests::device_code_login_completion_advances_to_success_message
        PASS [   0.073s] codex-tui onboarding::welcome::tests::welcome_renders_animation_on_first_draw
        PASS [   0.086s] codex-tui onboarding::welcome::tests::ctrl_dot_changes_animation_variant
        PASS [   0.058s] codex-tui oss_selection::tests::ctrl_h_l_move_provider_selection
        PASS [   0.082s] codex-tui onboarding::welcome::tests::welcome_skips_animation_below_height_breakpoint
        PASS [   0.072s] codex-tui pager_overlay::tests::edit_next_hint_is_visible_when_highlighted
        PASS [   0.079s] codex-tui pager_overlay::tests::edit_prev_hint_is_visible
        PASS [   0.073s] codex-tui pager_overlay::tests::pager_view_content_height_counts_renderables
        LEAK [   0.486s] codex-tui onboarding::auth::tests::existing_non_oauth_chatgpt_login_counts_as_signed_in
        PASS [   0.067s] codex-tui pager_overlay::tests::pager_view_ensure_chunk_visible_scrolls_down_when_needed
        PASS [   0.050s] codex-tui pager_overlay::tests::pager_view_ensure_chunk_visible_scrolls_up_when_needed
        PASS [   0.057s] codex-tui pager_overlay::tests::pager_view_is_scrolled_to_bottom_accounts_for_wrapped_height
        LEAK [   0.364s] codex-tui onboarding::auth::tests::saving_api_key_is_blocked_when_chatgpt_forced
        PASS [   0.070s] codex-tui pager_overlay::tests::transcript_overlay_consolidation_remaps_highlight_after_range
        PASS [   0.061s] codex-tui pager_overlay::tests::transcript_overlay_live_tail_preserves_semantic_web_links
        PASS [   0.101s] codex-tui pager_overlay::tests::transcript_overlay_consolidation_remaps_highlight_inside_range
        PASS [   0.132s] codex-tui pager_overlay::tests::transcript_overlay_keeps_scroll_pinned_at_bottom
        PASS [   0.081s] codex-tui pager_overlay::tests::transcript_overlay_preserves_manual_scroll_position
        PASS [   0.101s] codex-tui pager_overlay::tests::transcript_overlay_paging_is_continuous_and_round_trips
        PASS [   0.072s] codex-tui pager_overlay::tests::transcript_overlay_preserves_semantic_web_links
        PASS [   0.093s] codex-tui pager_overlay::tests::transcript_overlay_sync_live_tail_is_noop_for_identical_key
        PASS [   0.123s] codex-tui permission_compat::tests::compatibility_profile_preserves_unbridgeable_write_roots
        PASS [   0.105s] codex-tui pets::ambient::tests::animation_frame_uses_per_frame_duration
        PASS [   0.103s] codex-tui pets::ambient::tests::notification_labels_match_codex_app_vocabulary
        PASS [   0.070s] codex-tui pets::ambient::tests::reduced_motion_uses_stable_first_frame_and_schedules_no_follow_up
        PASS [   0.073s] codex-tui pets::asset_pack::tests::builtin_pet_url_uses_public_cdn_path
        PASS [   0.080s] codex-tui pets::image_protocol::tests::auto_protocol_is_disabled_inside_tmux
        PASS [   0.112s] codex-tui pets::frames::tests::prepare_png_frames_slices_spritesheet_without_external_command
        PASS [   0.095s] codex-tui pets::image_protocol::tests::explicit_protocol_still_resolves_inside_tmux
        PASS [   0.104s] codex-tui pets::image_protocol::tests::kitty_file_png_transmission_encodes_local_file_reference
        PASS [   0.813s] codex-tui onboarding::trust_directory::tests::renders_snapshot_for_git_repo
        PASS [   0.834s] codex-tui onboarding::trust_directory::tests::renders_snapshot_for_trust_error
        PASS [   0.075s] codex-tui pets::image_protocol::tests::parse_dotted_version_requires_simple_numeric_components
        PASS [   0.099s] codex-tui pets::image_protocol::tests::parses_protocol_selection
        PASS [   0.774s] codex-tui pager_overlay::tests::static_overlay_wraps_long_lines
        PASS [   0.107s] codex-tui pets::image_protocol::tests::kitty_png_transmission_encodes_inline_data
        PASS [   0.089s] codex-tui pets::image_protocol::tests::pet_image_support_detects_iterm2_kitty_file_graphics
        PASS [   0.800s] codex-tui pager_overlay::tests::static_overlay_snapshot_basic
        PASS [   0.050s] codex-tui pets::image_protocol::tests::pet_image_support_detects_kitty_graphics_terminals
        PASS [   0.053s] codex-tui pets::image_protocol::tests::pet_image_support_old_iterm2_message_mentions_upgrade
        PASS [   0.066s] codex-tui pets::image_protocol::tests::pet_image_support_detects_sixel_terminals
        PASS [   0.077s] codex-tui pets::image_protocol::tests::pet_image_support_prefers_multiplexer_safety
        PASS [   0.076s] codex-tui pets::image_protocol::tests::pet_image_support_rejects_old_iterm2_versions
        PASS [   0.081s] codex-tui pets::image_protocol::tests::pet_image_support_rejects_unknown_terminals
        PASS [   0.769s] codex-tui pager_overlay::tests::transcript_overlay_renders_live_tail
        PASS [   0.058s] codex-tui pets::image_protocol::tests::tmux_passthrough_wraps_and_escapes_control_sequence
        PASS [   0.071s] codex-tui pets::image_protocol::tests::wezterm_env_uses_kitty_graphics_for_ambient_pets
        PASS [   0.054s] codex-tui pets::model::tests::app_idle_animation_uses_calm_loop
        PASS [   0.069s] codex-tui pets::model::tests::app_notification_states_use_expected_rows
        PASS [   0.144s] codex-tui pets::image_protocol::tests::sixel_frame_encodes_without_external_crate
        PASS [   0.064s] codex-tui pets::model::tests::app_running_animation_repeats_then_settles_into_idle
        PASS [   0.053s] codex-tui pets::model::tests::custom_animation_specs_keep_manifest_fps_and_loop_shape
        PASS [   0.817s] codex-tui pager_overlay::tests::transcript_overlay_snapshot_basic
        PASS [   1.012s] codex-tui pager_overlay::tests::transcript_overlay_apply_patch_scroll_vt100_clears_previous_page
        PASS [   0.136s] codex-tui pets::model::tests::custom_pet_rejects_spritesheet_path_escape
        PASS [   1.343s] codex-tui pets::model::tests::custom_pet_rejects_animation_fallback_to_missing_animation
        PASS [   1.335s] codex-tui pets::model::tests::custom_pet_rejects_empty_animation_frames
        PASS [   1.319s] codex-tui pets::model::tests::custom_pet_rejects_frame_grid_that_does_not_cover_spritesheet
        PASS [   1.335s] codex-tui pets::model::tests::custom_pet_rejects_excessive_frame_count
        PASS [   1.337s] codex-tui pets::model::tests::custom_pet_rejects_zero_frame_dimensions
        PASS [   1.344s] codex-tui pets::model::tests::custom_pet_selector_falls_back_to_legacy_avatar_manifest
        PASS [   1.594s] codex-tui pets::model::tests::custom_pet_rejects_animation_frame_outside_grid
        PASS [   1.653s] codex-tui pets::model::tests::custom_pet_rejects_invalid_animation_fps
        PASS [   0.083s] codex-tui pets::picker::tests::picker_marks_disabled_pet_as_current
        PASS [   1.677s] codex-tui pets::model::tests::custom_pet_selector_loads_codex_home_pet_manifest
        PASS [   0.061s] codex-tui pets::picker::tests::picker_preselects_codex_without_marking_it_current_when_no_pet_is_configured
        PASS [   0.069s] codex-tui pets::preview::tests::centered_text_area_centers_vertically
        PASS [   0.082s] codex-tui pets::sixel::tests::encodes_red_pixel_with_palette_and_pixel_data
        PASS [   0.065s] codex-tui pets::sixel::tests::multi_band_images_advance_to_next_sixel_band
        PASS [   0.081s] codex-tui pets::sixel::tests::rejects_mismatched_rgba_buffer_length
        PASS [   0.080s] codex-tui pets::sixel::tests::repeated_cells_use_sixel_run_length_encoding
        PASS [   0.081s] codex-tui pets::sixel::tests::transparent_pixels_do_not_emit_palette_or_pixel_data
        PASS [   0.098s] codex-tui pets::tests::ambient_pet_image_restores_cursor_after_drawing
        PASS [   0.103s] codex-tui pets::tests::kitty_local_file_pet_image_uses_file_reference_without_inline_payload
        PASS [   0.077s] codex-tui pets::tests::kitty_pet_image_clear_deletes_without_moving_cursor
        PASS [   0.069s] codex-tui pets::tests::missing_frame_is_an_asset_error
        PASS [   0.165s] codex-tui pets::tests::sixel_pet_image_clear_erases_last_drawn_area
        PASS [   0.083s] codex-tui pets::tests::sixel_pet_image_clears_cell_area_before_redrawing
        PASS [   0.079s] codex-tui pets::tests::writer_failure_is_a_terminal_error
        PASS [   1.409s] codex-tui pets::model::tests::load_pet_directory_uses_app_pet_manifest_defaults
        PASS [   1.367s] codex-tui pets::picker::tests::picker_imports_legacy_avatar_manifests
        PASS [   0.064s] codex-tui render::highlight::tests::ansi_palette_color_maps_ansi_white_to_gray
        PASS [   0.338s] codex-tui render::highlight::tests::ansi_family_themes_use_terminal_palette_colors_not_rgb
        PASS [   0.081s] codex-tui render::highlight::tests::bundled_theme_can_provide_diff_scope_backgrounds
        PASS [   1.476s] codex-tui pets::model::tests::load_pet_json_path_uses_containing_directory
        PASS [   0.075s] codex-tui render::highlight::tests::convert_style_suppresses_underline
        PASS [   1.358s] codex-tui pets::picker::tests::picker_lists_app_bundled_and_custom_pets
        PASS [   0.068s] codex-tui render::highlight::tests::diff_scope_backgrounds_prefer_markup_scope_then_diff_fallback
        PASS [   0.080s] codex-tui render::highlight::tests::diff_scope_backgrounds_return_none_when_no_background_scope_matches
        PASS [   0.097s] codex-tui render::highlight::tests::custom_tmtheme_diff_scope_backgrounds_are_resolved
        PASS [   0.065s] codex-tui render::highlight::tests::foreground_style_for_scopes_reads_matching_theme_scope
        PASS [   0.056s] codex-tui render::highlight::tests::foreground_style_for_scopes_uses_first_scope_with_foreground
        PASS [   0.261s] codex-tui render::highlight::tests::fallback_trailing_newline_no_phantom_line
        PASS [   0.215s] codex-tui render::highlight::tests::find_syntax_resolves_languages_and_aliases
        PASS [   0.072s] codex-tui render::highlight::tests::highlight_empty_string
        PASS [   0.239s] codex-tui render::highlight::tests::highlight_bash_preserves_content
        PASS [   0.250s] codex-tui render::highlight::tests::highlight_code_to_styled_spans_returns_none_for_unknown
        PASS [   0.268s] codex-tui render::highlight::tests::highlight_code_to_styled_spans_returns_some_for_known
        PASS [   0.100s] codex-tui render::highlight::tests::highlight_many_lines_falls_back
        PASS [   0.103s] codex-tui render::highlight::tests::highlight_large_input_falls_back
        PASS [   0.077s] codex-tui render::highlight::tests::highlight_many_lines_no_trailing_newline_falls_back
        PASS [   0.899s] codex-tui render::highlight::tests::ansi_family_foreground_palette_snapshot
        PASS [   0.274s] codex-tui render::highlight::tests::highlight_crlf_strips_carriage_return
        PASS [   0.108s] codex-tui render::highlight::tests::list_available_themes_excludes_invalid_custom_files
        PASS [   0.112s] codex-tui render::highlight::tests::list_available_themes_returns_stable_sorted_order
        PASS [   0.071s] codex-tui render::highlight::tests::load_custom_theme_from_tmtheme_file
        PASS [   0.245s] codex-tui render::highlight::tests::highlight_multiline_python
        PASS [   0.246s] codex-tui render::highlight::tests::highlight_rust_has_keyword_style
        PASS [   0.299s] codex-tui render::highlight::tests::highlight_markdown_preserves_content
        PASS [   0.238s] codex-tui render::highlight::tests::highlight_unknown_lang_falls_back
        PASS [   0.066s] codex-tui render::highlight::tests::load_custom_theme_returns_none_for_missing
        PASS [   0.080s] codex-tui render::highlight::tests::parse_theme_name_covers_all_variants
        PASS [   0.085s] codex-tui render::highlight::tests::parse_theme_name_is_exhaustive
        PASS [   0.083s] codex-tui render::highlight::tests::parse_theme_name_returns_none_for_unknown
        PASS [   0.073s] codex-tui render::highlight::tests::style_conversion_correctness
        PASS [   0.058s] codex-tui render::highlight::tests::style_conversion_unexpected_alpha_falls_back_to_rgb
        PASS [   0.069s] codex-tui render::highlight::tests::style_conversion_uses_ansi_named_color_when_alpha_is_zero_low_index
        PASS [   0.086s] codex-tui render::highlight::tests::style_conversion_uses_indexed_color_when_alpha_is_zero_high_index
        PASS [   0.091s] codex-tui render::highlight::tests::style_conversion_uses_terminal_default_when_alpha_is_one
        PASS [   0.106s] codex-tui render::highlight::tests::validate_theme_name_none_for_bundled
        PASS [   0.109s] codex-tui render::highlight::tests::validate_theme_name_none_when_custom_file_is_valid
        PASS [   0.083s] codex-tui render::highlight::tests::validate_theme_name_none_when_no_override
        PASS [   0.071s] codex-tui render::highlight::tests::validate_theme_name_warns_for_missing_custom
        PASS [   0.062s] codex-tui resize_reflow_cap::tests::auto_resize_reflow_max_rows_prefers_vscode_probe
        PASS [   0.050s] codex-tui resize_reflow_cap::tests::auto_resize_reflow_max_rows_uses_terminal_defaults
        PASS [   0.096s] codex-tui render::highlight::tests::validate_theme_name_warns_when_custom_file_is_invalid
        PASS [   2.531s] codex-tui pets::model::tests::frame_cache_key_changes_with_frame_spec
        PASS [   0.062s] codex-tui resize_reflow_cap::tests::configured_resize_reflow_max_rows_overrides_auto_detection
        PASS [   0.056s] codex-tui resume_picker::tests::all_filter_can_switch_back_to_cwd_when_cwd_candidate_exists
        PASS [   0.091s] codex-tui resize_reflow_cap::tests::disabled_resize_reflow_max_rows_keeps_all_rows
        PASS [   0.069s] codex-tui resize_reflow_cap::tests::unknown_terminal_uses_fallback_even_under_multiplexer
        PASS [   0.073s] codex-tui resume_picker::tests::app_server_row_keeps_pathless_threads
        PASS [   0.065s] codex-tui resume_picker::tests::comfortable_zebra_lines_use_full_width_background
        PASS [   0.073s] codex-tui resume_picker::tests::cached_transcript_still_shows_loading_frame_before_opening_overlay
        PASS [   0.077s] codex-tui resume_picker::tests::ctrl_c_exits_even_when_cancel_is_remapped_to_ctrl_c
        PASS [   0.071s] codex-tui resume_picker::tests::ctrl_e_toggles_selected_session_expansion
        PASS [   0.084s] codex-tui resume_picker::tests::ctrl_o_keeps_toggled_density_when_persistence_fails
        PASS [   0.056s] codex-tui resume_picker::tests::ctrl_t_on_row_without_thread_id_shows_inline_error
        PASS [   0.101s] codex-tui resume_picker::tests::ctrl_o_persists_density_preference
        PASS [   0.072s] codex-tui resume_picker::tests::ctrl_o_toggles_density_without_typing_into_search
        PASS [   0.076s] codex-tui resume_picker::tests::ctrl_t_requests_selected_session_transcript
        PASS [   0.066s] codex-tui resume_picker::tests::default_filter_focus_arrows_reload_with_new_filter
        PASS [   0.054s] codex-tui resume_picker::tests::dense_selected_summary_line_uses_full_width_selection_style
        PASS [   0.065s] codex-tui resume_picker::tests::dense_session_line_prefers_thread_name_over_preview
        PASS [   0.050s] codex-tui resume_picker::tests::dense_zebra_summary_line_uses_full_width_background
        PASS [   0.088s] codex-tui resume_picker::tests::density_toggle_clears_stale_more_indicator
        PASS [   2.889s] codex-tui pets::model::tests::frame_cache_key_changes_with_spritesheet_contents
        PASS [   0.076s] codex-tui resume_picker::tests::end_jumps_to_last_known_row_and_starts_loading_more
        PASS [   0.071s] codex-tui resume_picker::tests::ensure_minimum_rows_does_not_prefetch_when_comfortable_cards_fill_view
        PASS [   0.071s] codex-tui resume_picker::tests::ensure_minimum_rows_prefetches_when_underfilled
        PASS [   0.094s] codex-tui resume_picker::tests::ensure_minimum_rows_still_prefetches_when_dense_rows_underfill_view
        PASS [   0.076s] codex-tui resume_picker::tests::enter_on_pathless_thread_uses_thread_id
        PASS [   0.063s] codex-tui resume_picker::tests::esc_with_empty_query_starts_fresh
        PASS [   0.061s] codex-tui resume_picker::tests::esc_with_query_clears_search_and_preserves_selected_result
        PASS [   0.071s] codex-tui resume_picker::tests::expanded_session_details_include_metadata
        PASS [   0.288s] codex-tui resume_picker::tests::enter_on_row_without_resolvable_thread_id_shows_inline_error
        PASS [   0.794s] codex-tui resume_picker::tests::dense_session_snapshot_omits_cwd_in_cwd_filter
        PASS [   0.075s] codex-tui resume_picker::tests::filter_stays_all_when_no_cwd_candidate_exists
        PASS [   0.824s] codex-tui resume_picker::tests::dense_session_snapshot_drops_metadata_when_narrow
        PASS [   0.828s] codex-tui resume_picker::tests::dense_session_snapshot_includes_cwd_in_all_filter
        PASS [   0.872s] codex-tui resume_picker::tests::dense_session_snapshot_auto_hides_cwd_when_narrow
        PASS [   0.863s] codex-tui resume_picker::tests::dense_session_snapshot_forces_cwd_when_narrow
        PASS [   0.063s] codex-tui resume_picker::tests::footer_branch_expands_when_line_has_room
        PASS [   0.844s] codex-tui resume_picker::tests::dense_session_snapshot_uses_no_blank_lines_between_rows
        PASS [   0.063s] codex-tui resume_picker::tests::footer_cwd_truncates_to_responsive_column
        PASS [   0.073s] codex-tui resume_picker::tests::footer_marks_missing_branch
        PASS [   0.069s] codex-tui resume_picker::tests::footer_omits_cwd_when_hidden
        PASS [   0.057s] codex-tui resume_picker::tests::footer_prioritizes_active_sort_timestamp
        PASS [   0.054s] codex-tui resume_picker::tests::hint_line_compacts_on_narrow_width
        PASS [   0.053s] codex-tui resume_picker::tests::hint_line_labels_cancel_keys_as_exit_for_existing_session_resume_picker
        PASS [   0.070s] codex-tui resume_picker::tests::hint_line_shows_loading_transcript_mode
        PASS [   0.070s] codex-tui resume_picker::tests::hint_line_prioritizes_keybinds_when_very_narrow
        PASS [   0.075s] codex-tui resume_picker::tests::hint_line_switches_density_label
        PASS [   0.085s] codex-tui resume_picker::tests::hint_line_switches_esc_label_for_search_mode
        PASS [   0.071s] codex-tui resume_picker::tests::list_reports_more_rows_above_and_below
        PASS [   0.067s] codex-tui resume_picker::tests::list_viewport_width_matches_rendered_list_inset
        PASS [   0.074s] codex-tui resume_picker::tests::loaded_transcript_waits_for_loading_frame_before_opening_overlay
        PASS [   0.056s] codex-tui resume_picker::tests::local_picker_thread_list_params_include_cwd_filter
        PASS [   0.068s] codex-tui resume_picker::tests::long_relative_time_uses_words
        PASS [   0.083s] codex-tui resume_picker::tests::moving_to_last_card_scrolls_when_cards_exceed_viewport
        PASS [   0.082s] codex-tui resume_picker::tests::page_and_jump_navigation_use_list_keymap
        PASS [   0.074s] codex-tui resume_picker::tests::page_navigation_uses_view_rows
        PASS [   0.072s] codex-tui resume_picker::tests::pageless_scrolling_deduplicates_and_keeps_order
        PASS [   0.064s] codex-tui resume_picker::tests::paste_uses_existing_search_loading_path
        PASS [   0.085s] codex-tui resume_picker::tests::paste_appends_to_existing_query
        PASS [   0.069s] codex-tui resume_picker::tests::picker_footer_percent_is_complete_when_not_scrollable
        PASS [   0.061s] codex-tui resume_picker::tests::picker_footer_percent_reports_scroll_progress
        PASS [   0.064s] codex-tui resume_picker::tests::picker_footer_progress_label_shows_position_total_and_percent
        PASS [   0.065s] codex-tui resume_picker::tests::picker_footer_progress_label_uses_known_count_when_more_pages_exist
        PASS [   0.087s] codex-tui resume_picker::tests::picker_footer_progress_label_freezes_percent_while_loading
        PASS [   0.082s] codex-tui resume_picker::tests::raw_ctrl_e_toggles_selected_session_expansion
        PASS [   0.089s] codex-tui resume_picker::tests::raw_ctrl_o_toggles_density_without_typing_into_search
        PASS [   0.067s] codex-tui resume_picker::tests::relative_time_formats_zero_seconds_as_now
        PASS [   0.069s] codex-tui resume_picker::tests::remote_picker_does_not_filter_rows_by_local_cwd
        PASS [   0.094s] codex-tui resume_picker::tests::raw_ctrl_t_requests_selected_session_transcript
        PASS [   0.057s] codex-tui resume_picker::tests::remote_thread_list_params_can_include_non_interactive_sources
        PASS [   0.081s] codex-tui resume_picker::tests::remote_picker_sends_cwd_filter_without_local_post_filtering
        PASS [   0.068s] codex-tui resume_picker::tests::remote_thread_list_params_omit_provider_filter
        PASS [   0.846s] codex-tui resume_picker::tests::expanded_session_snapshot
        PASS [   0.071s] codex-tui resume_picker::tests::row_display_preview_prefers_thread_name
        PASS [   0.076s] codex-tui resume_picker::tests::row_search_matches_metadata_fields
        PASS [   0.128s] codex-tui resume_picker::tests::search_line_compacts_toolbar_on_narrow_width
        PASS [   0.795s] codex-tui resume_picker::tests::hint_line_snapshot_uses_compact_footer
        PASS [   0.842s] codex-tui resume_picker::tests::hint_line_snapshot_uses_distributed_wide_footer
        PASS [   0.073s] codex-tui resume_picker::tests::set_query_loads_until_match_and_respects_scan_cap
        PASS [   0.097s] codex-tui resume_picker::tests::space_appends_to_search_query
        PASS [   0.805s] codex-tui resume_picker::tests::narrow_session_snapshot
        PASS [   0.097s] codex-tui resume_picker::tests::thread_to_transcript_cells_hides_raw_reasoning_when_not_enabled
        PASS [   0.095s] codex-tui resume_picker::tests::thread_to_transcript_cells_renders_core_message_types
        PASS [   0.095s] codex-tui resume_picker::tests::thread_to_transcript_cells_shows_raw_reasoning_over_summary_when_enabled
        PASS [   0.111s] codex-tui resume_picker::tests::toggle_sort_key_reloads_with_new_sort
        PASS [   0.100s] codex-tui resume_picker::tests::transcript_loading_consumes_picker_input
        PASS [   0.066s] codex-tui resume_picker::tests::transcript_loading_still_allows_ctrl_c_exit
        PASS [   0.076s] codex-tui resume_picker::tests::up_scrolls_only_after_crossing_top_edge
        PASS [   0.082s] codex-tui resume_picker::tests::up_from_bottom_keeps_viewport_stable_when_card_remains_visible
        PASS [   0.059s] codex-tui resume_picker::tests::whitespace_only_paste_is_ignored
        PASS [   0.091s] codex-tui session_resume::tests::rollout_resume_state_falls_back_to_session_meta
        PASS [   0.080s] codex-tui session_resume::tests::rollout_resume_state_prefers_latest_turn_context
        PASS [   0.063s] codex-tui slash_command::tests::auto_review_command_is_approve
        PASS [   0.110s] codex-tui session_resume::tests::rollout_resume_state_skips_malformed_lines
        PASS [   0.775s] codex-tui resume_picker::tests::resume_table_snapshot
        PASS [   0.072s] codex-tui slash_command::tests::certain_commands_are_available_during_task
        PASS [   0.818s] codex-tui resume_picker::tests::resume_search_error_snapshot
        PASS [   0.048s] codex-tui slash_command::tests::pet_alias_parses_to_pets_command
        PASS [   0.065s] codex-tui slash_command::tests::clean_alias_parses_to_stop_command
        PASS [   0.078s] codex-tui startup_hooks_review::tests::bypass_hook_trust_suppresses_startup_review
        PASS [   0.080s] codex-tui slash_command::tests::stop_command_is_canonical_name
        PASS [   0.805s] codex-tui resume_picker::tests::session_list_more_indicators_snapshot
        PASS [   0.807s] codex-tui resume_picker::tests::search_line_renders_sort_and_filter_tabs
        PASS [   0.090s] codex-tui startup_hooks_review::tests::untrusted_hooks_need_review_without_bypass
  TRY 1 FAIL [   0.087s] codex-tui status::helpers::tests::compose_agents_summary_includes_global_agents_path
  stdout ---

    running 1 test
    test status::helpers::tests::compose_agents_summary_includes_global_agents_path ... FAILED

    failures:

    failures:
        status::helpers::tests::compose_agents_summary_includes_global_agents_path

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.03s

  stderr ---

    thread 'status::helpers::tests::compose_agents_summary_includes_global_agents_path' (14472) panicked at tui\src\status\helpers.rs:233:9:
    assertion failed: `(left == right)`

    Diff < left / right > :
    <C:\Users\JohnDell\AppData\Local\Temp\.tmpOvPguL\AGENTS.md
    >~\AppData\Local\Temp\.tmpOvPguL\AGENTS.md


    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

  TRY 1 FAIL [   0.113s] codex-tui status::helpers::tests::compose_agents_summary_names_global_agents_override
  stdout ---

    running 1 test
    test status::helpers::tests::compose_agents_summary_names_global_agents_override ... FAILED

    failures:

    failures:
        status::helpers::tests::compose_agents_summary_names_global_agents_override

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.04s

  stderr ---

    thread 'status::helpers::tests::compose_agents_summary_names_global_agents_override' (25852) panicked at tui\src\status\helpers.rs:246:9:
    assertion failed: `(left == right)`

    Diff < left / right > :
    <C:\Users\JohnDell\AppData\Local\Temp\.tmp9y510y\AGENTS.override.md
    >~\AppData\Local\Temp\.tmp9y510y\AGENTS.override.md


    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.069s] codex-tui status::helpers::tests::plan_type_display_name_remaps_display_labels
        PASS [   0.080s] codex-tui status::rate_limits::tests::non_codex_multi_limit_keeps_group_row
  TRY 1 FAIL [   0.106s] codex-tui status::helpers::tests::compose_agents_summary_orders_global_before_project_agents
  stdout ---

    running 1 test
    test status::helpers::tests::compose_agents_summary_orders_global_before_project_agents ... FAILED

    failures:

    failures:
        status::helpers::tests::compose_agents_summary_orders_global_before_project_agents

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.04s

  stderr ---

    thread 'status::helpers::tests::compose_agents_summary_orders_global_before_project_agents' (16112) panicked at tui\src\status\helpers.rs:268:9:
    assertion failed: `(left == right)`

    Diff < left / right > :
     Some(
    <    "C:\\Users\\JohnDell\\AppData\\Local\\Temp\\.tmpfsmMsN\\AGENTS.md",
    >    "~\\AppData\\Local\\Temp\\.tmpfsmMsN\\AGENTS.md",
     )


    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

  TRY 2 FAIL [   0.106s] codex-tui status::helpers::tests::compose_agents_summary_includes_global_agents_path
  stdout ---

    running 1 test
    test status::helpers::tests::compose_agents_summary_includes_global_agents_path ... FAILED

    failures:

    failures:
        status::helpers::tests::compose_agents_summary_includes_global_agents_path

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.04s

  stderr ---

    thread 'status::helpers::tests::compose_agents_summary_includes_global_agents_path' (16744) panicked at tui\src\status\helpers.rs:233:9:
    assertion failed: `(left == right)`

    Diff < left / right > :
    <C:\Users\JohnDell\AppData\Local\Temp\.tmpYot2xW\AGENTS.md
    >~\AppData\Local\Temp\.tmpYot2xW\AGENTS.md


    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

  TRY 2 FAIL [   0.078s] codex-tui status::helpers::tests::compose_agents_summary_names_global_agents_override
  stdout ---

    running 1 test
    test status::helpers::tests::compose_agents_summary_names_global_agents_override ... FAILED

    failures:

    failures:
        status::helpers::tests::compose_agents_summary_names_global_agents_override

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.03s

  stderr ---

    thread 'status::helpers::tests::compose_agents_summary_names_global_agents_override' (17748) panicked at tui\src\status\helpers.rs:246:9:
    assertion failed: `(left == right)`

    Diff < left / right > :
    <C:\Users\JohnDell\AppData\Local\Temp\.tmpfp34D0\AGENTS.override.md
    >~\AppData\Local\Temp\.tmpfp34D0\AGENTS.override.md


    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.067s] codex-tui status::rate_limits::tests::non_codex_single_limit_renders_combined_row
        PASS [   0.066s] codex-tui status::remote_connection::tests::remote_connection_status_value_formats_display_value
        PASS [   0.055s] codex-tui status::tests::stale_monthly_limit_marks_fresh_rolling_snapshot_stale
  TRY 2 FAIL [   0.067s] codex-tui status::helpers::tests::compose_agents_summary_orders_global_before_project_agents
  stdout ---

    running 1 test
    test status::helpers::tests::compose_agents_summary_orders_global_before_project_agents ... FAILED

    failures:

    failures:
        status::helpers::tests::compose_agents_summary_orders_global_before_project_agents

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2791 filtered out; finished in 0.02s

  stderr ---

    thread 'status::helpers::tests::compose_agents_summary_orders_global_before_project_agents' (8948) panicked at tui\src\status\helpers.rs:268:9:
    assertion failed: `(left == right)`

    Diff < left / right > :
     Some(
    <    "C:\\Users\\JohnDell\\AppData\\Local\\Temp\\.tmp9C77lM\\AGENTS.md",
    >    "~\\AppData\\Local\\Temp\\.tmp9C77lM\\AGENTS.md",
     )


    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        PASS [   0.102s] codex-tui status::tests::status_model_provider_uses_bedrock_runtime_base_url_and_gates_usage_link
        PASS [   0.113s] codex-tui status::tests::status_card_token_usage_excludes_cached_tokens
        PASS [   0.095s] codex-tui status::tests::status_permissions_broadened_workspace_profile_shows_builtin_label
        PASS [   0.120s] codex-tui status::tests::status_context_window_uses_last_usage
        PASS [   0.101s] codex-tui status::tests::status_permissions_full_disk_managed_with_network_is_danger_full_access
        PASS [   0.737s] codex-tui resume_picker::tests::transcript_loading_overlay_snapshot
        PASS [   0.113s] codex-tui status::tests::status_permissions_full_disk_managed_without_network_is_external_sandbox
        PASS [   0.112s] codex-tui status::tests::status_permissions_named_read_only_profile_shows_builtin_label
        PASS [   0.118s] codex-tui status::tests::status_permissions_named_profile_shows_additional_writable_roots
        PASS [   0.113s] codex-tui status::tests::status_permissions_named_workspace_profile_shows_builtin_label
        PASS [   0.133s] codex-tui status::tests::status_permissions_non_default_workspace_write_uses_workspace_label
        PASS [   0.101s] codex-tui status::tests::status_permissions_user_defined_profile_shows_name
        PASS [   0.084s] codex-tui status::tests::status_permissions_workspace_auto_review_shows_reviewer_label
        PASS [   0.141s] codex-tui status::tests::status_permissions_read_only_profile_shows_additional_writable_roots
        PASS [   0.091s] codex-tui status::tests::status_permissions_workspace_roots_include_profile_defined_directories
        PASS [   0.103s] codex-tui status::tests::status_permissions_workspace_roots_show_additional_directories
        PASS [   0.080s] codex-tui status::tests::status_snapshot_hides_when_has_no_credits_flag
        PASS [   0.097s] codex-tui status::tests::status_snapshot_hides_zero_credits
        PASS [   0.788s] codex-tui startup_hooks_review::tests::renders_prompt
        PASS [   0.758s] codex-tui startup_hooks_review::tests::renders_prompt_with_trust_error
        PASS [   0.859s] codex-tui status::tests::status_snapshot_includes_credits_and_limits
        PASS [   0.874s] codex-tui status::tests::status_snapshot_includes_monthly_limit
        PASS [   0.967s] codex-tui status::tests::status_snapshot_cached_limits_hide_credits_without_flag
        PASS [   0.965s] codex-tui status::tests::status_snapshot_includes_enterprise_monthly_credit_limit
        PASS [   0.872s] codex-tui status::tests::status_snapshot_includes_reasoning_details
        PASS [   0.834s] codex-tui status::tests::status_snapshot_shows_active_user_defined_profile
        PASS [   0.951s] codex-tui status::tests::status_snapshot_includes_forked_from
        PASS [   0.819s] codex-tui status::tests::status_snapshot_shows_auto_review_permissions
        PASS [   0.080s] codex-tui status::tests::status_snapshot_shows_positive_credits
        PASS [   0.129s] codex-tui status::tests::status_snapshot_shows_unlimited_credits
        PASS [  21.472s] codex-tui diff_render::tests::large_update_diff_skips_highlighting
        PASS [   0.104s] codex-tui status_indicator_widget::tests::details_args_can_disable_capitalization_and_limit_lines
        PASS [   0.110s] codex-tui status_indicator_widget::tests::details_overflow_adds_ellipsis
        PASS [   0.070s] codex-tui status_indicator_widget::tests::fmt_elapsed_compact_formats_seconds_minutes_hours
        PASS [   0.767s] codex-tui status::tests::status_snapshot_shows_missing_limits_message
        PASS [   0.818s] codex-tui status::tests::status_snapshot_shows_refreshing_limits_notice
        PASS [   0.841s] codex-tui status::tests::status_snapshot_shows_stale_limits_message
        PASS [   0.891s] codex-tui status::tests::status_snapshot_shows_unavailable_limits_message
        PASS [   0.868s] codex-tui status::tests::status_snapshot_treats_refreshing_empty_limits_as_unavailable
        PASS [   0.874s] codex-tui status::tests::status_snapshot_truncates_in_narrow_terminal
        PASS [   0.916s] codex-tui status::tests::status_snapshot_uses_default_reasoning_when_config_empty
        PASS [   0.850s] codex-tui status::tests::status_snapshot_uses_generic_limit_labels_for_unsupported_windows
        PASS [   0.080s] codex-tui status_indicator_widget::tests::renders_without_spinner_when_animations_disabled
        PASS [   0.081s] codex-tui status_indicator_widget::tests::timer_pauses_when_requested
        PASS [   0.093s] codex-tui streaming::chunking::tests::catch_up_batch_drains_current_backlog
        PASS [   0.052s] codex-tui streaming::chunking::tests::enters_catch_up_on_age_threshold
        PASS [   0.069s] codex-tui streaming::chunking::tests::drops_back_to_smooth_when_idle
        PASS [   0.064s] codex-tui streaming::chunking::tests::enters_catch_up_on_depth_threshold
        PASS [   0.090s] codex-tui streaming::chunking::tests::exits_catch_up_after_hysteresis_hold
        PASS [   0.099s] codex-tui streaming::chunking::tests::holds_reentry_after_catch_up_exit
        PASS [   0.089s] codex-tui streaming::chunking::tests::severe_backlog_uses_faster_paced_batches
        PASS [   0.093s] codex-tui streaming::chunking::tests::smooth_mode_is_default
        PASS [   0.105s] codex-tui streaming::chunking::tests::severe_backlog_can_reenter_during_hold
        PASS [   0.089s] codex-tui streaming::controller::tests::controller_does_not_hold_back_pipe_prose_without_table_delimiter
        PASS [   0.079s] codex-tui streaming::controller::tests::controller_does_not_stall_repeated_pipe_prose_paragraphs
        PASS [   0.164s] codex-tui streaming::controller::tests::controller_converts_no_outer_table_between_preboxed_sections
        PASS [   0.103s] codex-tui streaming::controller::tests::controller_has_live_tail_reflects_tail_presence
        PASS [   0.132s] codex-tui streaming::controller::tests::controller_handles_table_immediately_after_heading
        PASS [   0.142s] codex-tui streaming::controller::tests::controller_holds_blockquoted_table_tail_until_stable
        PASS [   0.097s] codex-tui streaming::controller::tests::controller_keeps_pre_table_lines_queued_when_table_is_confirmed
        PASS [   0.142s] codex-tui streaming::controller::tests::controller_live_tail_requires_table_holdback_state
        PASS [   0.310s] codex-tui streaming::controller::tests::controller_keeps_markdown_fenced_no_outer_tables_mutable_until_finalize
        PASS [   0.184s] codex-tui streaming::controller::tests::controller_live_tail_keeps_uncommitted_table_cell_newline_gated
        PASS [   0.891s] codex-tui status_indicator_widget::tests::renders_remapped_interrupt_hint
        PASS [   0.890s] codex-tui status_indicator_widget::tests::renders_truncated
        PASS [   0.350s] codex-tui streaming::controller::tests::controller_keeps_markdown_fenced_tables_mutable_until_finalize
        PASS [   0.343s] codex-tui streaming::controller::tests::controller_keeps_non_markdown_fenced_tables_as_code
        PASS [   0.109s] codex-tui streaming::controller::tests::controller_live_tail_rerenders_table_tail_after_resize
        PASS [   0.073s] codex-tui streaming::controller::tests::controller_loose_vs_tight_with_commit_ticks_matches_full
        PASS [   0.060s] codex-tui streaming::controller::tests::controller_set_width_after_first_line_emit_does_not_requeue_first_line
        PASS [   0.902s] codex-tui status_indicator_widget::tests::renders_with_working_header
        PASS [   0.153s] codex-tui streaming::controller::tests::controller_renders_separators_for_multi_table_response_shape
        PASS [   0.084s] codex-tui streaming::controller::tests::controller_set_width_partial_drain_keeps_pending_queue
        PASS [   0.056s] codex-tui streaming::controller::tests::controller_set_width_partial_drain_no_lost_lines
        PASS [   0.176s] codex-tui streaming::controller::tests::controller_live_view_matches_render_during_interleaved_table_streaming
        PASS [   0.094s] codex-tui streaming::controller::tests::controller_set_width_during_confirmed_table_stream_matches_finalize_render
        PASS [   0.143s] codex-tui streaming::controller::tests::controller_renders_separators_for_no_outer_pipes_table_shape
        PASS [   0.104s] codex-tui streaming::controller::tests::controller_set_width_no_duplicate_after_emit
        PASS [   0.066s] codex-tui streaming::controller::tests::controller_set_width_partial_wrapped_emit_keeps_wrapped_remainder
        PASS [   0.057s] codex-tui streaming::controller::tests::controller_set_width_partial_wrapped_emit_preserves_remaining_content
        PASS [   0.054s] codex-tui streaming::controller::tests::controller_set_width_preserves_in_flight_tail
        PASS [   0.889s] codex-tui status_indicator_widget::tests::renders_wrapped_details_panama_two_lines
        PASS [   0.063s] codex-tui streaming::controller::tests::controller_set_width_preserves_table_tail_when_queue_is_empty
        PASS [   0.087s] codex-tui streaming::controller::tests::controller_set_width_rebuilds_queued_lines
        PASS [   0.076s] codex-tui streaming::controller::tests::controller_tick_batch_zero_is_noop
        PASS [   0.116s] codex-tui streaming::controller::tests::controller_stabilizes_first_no_outer_pipes_table_in_response
        PASS [   0.117s] codex-tui streaming::controller::tests::controller_stabilizes_two_column_no_outer_table_in_response
        PASS [   0.122s] codex-tui streaming::controller::tests::controller_streamed_table_matches_full_render_widths
        PASS [  10.234s] codex-tui pets::asset_pack::tests::write_test_pack_installs_all_builtins
        PASS [   0.092s] codex-tui streaming::controller::tests::incremental_holdback_detects_header_delimiter_across_chunk_boundary
        PASS [   0.084s] codex-tui streaming::controller::tests::incremental_holdback_matches_stateless_scan_per_chunk
        PASS [   0.124s] codex-tui streaming::controller::tests::finalized_stream_table_preserves_semantic_url_fragments
        PASS [   0.132s] codex-tui streaming::controller::tests::finalized_plan_stream_preserves_semantic_url_fragments
        PASS [   0.067s] codex-tui streaming::controller::tests::plan_controller_has_live_tail_reflects_tail_presence
        PASS [   0.089s] codex-tui streaming::controller::tests::plan_controller_holds_table_header_as_live_tail
        PASS [   0.093s] codex-tui streaming::controller::tests::plan_controller_set_width_preserves_in_flight_tail
        PASS [   0.081s] codex-tui streaming::controller::tests::table_holdback_state_detects_single_column_header_plus_delimiter
        PASS [   0.101s] codex-tui streaming::controller::tests::table_holdback_state_detects_header_plus_delimiter
        PASS [   0.098s] codex-tui streaming::controller::tests::table_holdback_state_ignores_table_like_lines_inside_blockquoted_other_fence
        PASS [   0.064s] codex-tui streaming::controller::tests::table_holdback_state_ignores_table_like_lines_inside_unclosed_long_fence
        PASS [   0.076s] codex-tui streaming::controller::tests::table_holdback_state_treats_indented_fence_text_as_plain_content
        PASS [   0.137s] codex-tui streaming::controller::tests::plan_controller_streamed_table_matches_final_render
        PASS [   0.084s] codex-tui streaming::tests::drain_n_clamps_to_available_lines
        PASS [   0.053s] codex-tui style::tests::table_separator_blends_toward_dark_background
        PASS [   0.083s] codex-tui style::tests::accent_style_uses_cyan_on_dark_or_unknown_backgrounds
        PASS [   0.084s] codex-tui style::tests::accent_style_uses_darker_cyan_on_light_backgrounds
        PASS [   0.079s] codex-tui style::tests::table_separator_blends_toward_light_background
        PASS [   0.068s] codex-tui style::tests::table_separator_dims_when_palette_aware_color_is_unavailable
        PASS [   0.057s] codex-tui table_detect::tests::fence_tracker_blockquote_prefix_stripped
        PASS [   0.071s] codex-tui table_detect::tests::fence_tracker_indented_4_spaces_ignored
        PASS [   0.078s] codex-tui table_detect::tests::fence_tracker_close_with_trailing_content_does_not_close
        PASS [   0.081s] codex-tui table_detect::tests::fence_tracker_markdown_case_insensitive
        PASS [   0.076s] codex-tui table_detect::tests::fence_tracker_markdown_fence
        PASS [   0.078s] codex-tui table_detect::tests::fence_tracker_nested_shorter_marker_does_not_close
        PASS [   0.080s] codex-tui table_detect::tests::fence_tracker_mismatched_char_does_not_close
        PASS [   0.072s] codex-tui table_detect::tests::fence_tracker_opens_and_closes_backtick_fence
        PASS [   0.077s] codex-tui table_detect::tests::fence_tracker_opens_and_closes_tilde_fence
        PASS [   0.087s] codex-tui table_detect::tests::fence_tracker_outside_by_default
        PASS [   0.085s] codex-tui table_detect::tests::is_markdown_fence_info_basic
        PASS [   0.069s] codex-tui table_detect::tests::is_table_delimiter_line_valid
        PASS [   0.084s] codex-tui table_detect::tests::is_table_delimiter_line_invalid
        PASS [   0.313s] codex-tui streaming::controller::tests::plan_controller_streamed_markdown_fenced_table_matches_final_render
        PASS [   0.079s] codex-tui table_detect::tests::is_table_delimiter_segment_invalid
        PASS [   0.073s] codex-tui table_detect::tests::is_table_header_line_all_empty_segments
        PASS [   0.070s] codex-tui table_detect::tests::is_table_delimiter_segment_valid
        PASS [   0.076s] codex-tui table_detect::tests::is_table_header_line_valid
        PASS [   0.081s] codex-tui table_detect::tests::parse_fence_marker_backtick
        PASS [   0.067s] codex-tui table_detect::tests::parse_fence_marker_not_fence
        PASS [   0.064s] codex-tui table_detect::tests::parse_fence_marker_tilde
        PASS [   0.069s] codex-tui table_detect::tests::parse_fence_marker_too_short
        PASS [   0.085s] codex-tui table_detect::tests::parse_table_segments_basic
        PASS [   0.081s] codex-tui table_detect::tests::parse_table_segments_empty_returns_none
        PASS [   0.084s] codex-tui table_detect::tests::parse_table_segments_escaped_pipe
        PASS [   0.085s] codex-tui table_detect::tests::parse_table_segments_no_outer_pipes
        PASS [   0.087s] codex-tui table_detect::tests::parse_table_segments_no_leading_pipe
        PASS [   0.086s] codex-tui table_detect::tests::parse_table_segments_no_trailing_pipe
        PASS [   0.082s] codex-tui table_detect::tests::parse_table_segments_single_segment_is_allowed
        PASS [   0.079s] codex-tui table_detect::tests::parse_table_segments_without_pipe_returns_none
        PASS [   0.086s] codex-tui table_detect::tests::strip_blockquote_prefix_basic
        PASS [   0.088s] codex-tui terminal_hyperlinks::tests::buffer_hyperlinks_follow_word_wrapping
        PASS [   0.077s] codex-tui terminal_hyperlinks::tests::decorates_a_contiguous_web_link_with_one_osc8_pair
        PASS [   0.082s] codex-tui terminal_hyperlinks::tests::discovers_punctuated_web_url_columns
        PASS [   0.077s] codex-tui terminal_hyperlinks::tests::only_web_destinations_receive_osc8
        PASS [   0.078s] codex-tui terminal_hyperlinks::tests::preserves_balanced_parentheses_in_bare_web_urls
        PASS [   0.094s] codex-tui terminal_hyperlinks::tests::wrapping_maps_repeated_link_labels_by_source_position
        PASS [   0.090s] codex-tui terminal_palette::tests::best_color_resets_for_ansi16
        PASS [   0.092s] codex-tui terminal_palette::tests::best_color_uses_truecolor_without_quantization
        PASS [   0.100s] codex-tui terminal_palette::tests::force_color_keeps_reported_stdout_level
        PASS [   0.087s] codex-tui terminal_palette::tests::windows_terminal_name_promotes_ansi16_to_truecolor
        PASS [   0.087s] codex-tui terminal_palette::tests::windows_terminal_wt_session_promotes_to_truecolor
        PASS [   0.099s] codex-tui terminal_probe::imp::tests::decodes_console_color_attribute_indices
        PASS [   0.092s] codex-tui terminal_probe::imp::tests::decodes_console_color_intensity_indices
        PASS [   0.091s] codex-tui terminal_probe::imp::tests::decodes_console_color_ref_byte_order
        PASS [   0.091s] codex-tui terminal_probe::imp::tests::ignores_reverse_video_when_decoding_default_colors
        PASS [   0.085s] codex-tui terminal_probe::tests::ignores_malformed_or_partial_default_color_responses
        PASS [   0.091s] codex-tui terminal_probe::tests::parses_default_colors_from_one_buffer
        PASS [   0.082s] codex-tui terminal_probe::tests::parses_default_colors_with_unrelated_bytes
        PASS [   0.080s] codex-tui terminal_probe::tests::parses_osc_colors_with_bel_and_st
        PASS [   0.076s] codex-tui terminal_probe::tests::parses_two_and_four_digit_color_components
        PASS [   0.077s] codex-tui terminal_title::tests::sanitizes_terminal_title
        PASS [   0.075s] codex-tui terminal_title::tests::strips_invisible_format_chars_from_terminal_title
        PASS [   0.083s] codex-tui terminal_title::tests::truncates_terminal_title
        PASS [   0.079s] codex-tui terminal_title::tests::truncation_prefers_visible_char_over_pending_space
        PASS [   0.083s] codex-tui terminal_title::tests::writes_osc_title_with_bel_terminator
        PASS [   0.074s] codex-tui tests::alternate_screen_auto_uses_alt_screen
        PASS [   0.079s] codex-tui tests::app_server_target_for_launch_prefers_explicit_remote_endpoint
        PASS [   0.085s] codex-tui tests::app_server_target_for_launch_skips_local_daemon_when_launch_config_is_not_replayable
        PASS [   0.081s] codex-tui tests::app_server_target_for_launch_uses_local_daemon_for_default_socket
        PASS [   0.088s] codex-tui tests::can_reuse_implicit_local_daemon_requires_default_launch_config
        PASS [   0.083s] codex-tui tests::config_cwd_for_app_server_target_canonicalizes_embedded_cli_cwd
        PASS [   0.086s] codex-tui tests::config_cwd_for_app_server_target_canonicalizes_local_daemon_cli_cwd
        PASS [   0.079s] codex-tui tests::config_cwd_for_app_server_target_errors_for_missing_embedded_cli_cwd
        PASS [   0.163s] codex-tui tests::config_cwd_for_app_server_target_omits_cwd_for_remote_exec_server
        PASS [   0.156s] codex-tui tests::config_cwd_for_app_server_target_omits_cwd_for_remote_sessions
        PASS [   0.156s] codex-tui tests::default_daemon_auto_connect_skips_missing_socket
        PASS [   0.174s] codex-tui tests::embedded_app_server_start_failure_is_returned
        PASS [   0.209s] codex-tui tests::config_rebuild_changes_trust_defaults_with_cwd
        PASS [   0.186s] codex-tui tests::embedded_state_db_failure_is_typed_for_cli_recovery
        PASS [   0.210s] codex-tui tests::latest_session_cwd_filter_respects_scope_options
        PASS [   0.158s] codex-tui tests::latest_session_lookup_params_can_include_non_interactive_sources
        PASS [   0.157s] codex-tui tests::latest_session_lookup_params_keep_local_filters_for_embedded_sessions
        PASS [   0.174s] codex-tui tests::latest_session_lookup_params_keep_explicit_cwd_filter_for_remote_sessions
        PASS [   0.117s] codex-tui tests::latest_session_lookup_params_omit_local_filters_for_remote_sessions
        PASS [   0.145s] codex-tui tests::latest_session_lookup_params_keep_local_filters_for_local_daemon_sessions
        PASS [   0.082s] codex-tui tests::resolve_remote_addr_accepts_absolute_socket_path
        PASS [   0.081s] codex-tui tests::resolve_remote_addr_accepts_default_socket
        PASS [   0.085s] codex-tui tests::resolve_remote_addr_accepts_secure_websocket_url
        PASS [   0.090s] codex-tui tests::resolve_remote_addr_accepts_relative_socket_path
        PASS [   0.087s] codex-tui tests::resolve_remote_addr_accepts_websocket_url
        PASS [   0.092s] codex-tui tests::resolve_remote_addr_rejects_invalid_remote_addresses
        PASS [   0.094s] codex-tui tests::session_target_display_label_falls_back_to_thread_id
        PASS [   0.065s] codex-tui tests::should_load_configured_environments_for_local_daemon
        PASS [   0.065s] codex-tui tests::startup_removes_legacy_tui_log_file
        PASS [   0.097s] codex-tui tests::theme_warning_uses_final_config
        PASS [   0.108s] codex-tui tests::untrusted_project_skips_trust_prompt
        PASS [   0.054s] codex-tui text_formatting::tests::test_center_truncate_handles_long_segment
        PASS [   0.085s] codex-tui tests::windows_shows_trust_prompt_with_sandbox
        PASS [   0.055s] codex-tui text_formatting::tests::test_center_truncate_doesnt_truncate_short_path
        PASS [   0.097s] codex-tui tests::windows_shows_trust_prompt_without_sandbox
        PASS [   0.050s] codex-tui text_formatting::tests::test_center_truncate_truncates_long_path
        PASS [   0.067s] codex-tui text_formatting::tests::test_center_truncate_truncates_long_windows_path
        PASS [   0.071s] codex-tui text_formatting::tests::test_format_json_compact_already_compact
        PASS [   0.077s] codex-tui text_formatting::tests::test_format_json_compact_empty_array
        PASS [   0.079s] codex-tui text_formatting::tests::test_format_json_compact_array
        PASS [   0.083s] codex-tui text_formatting::tests::test_format_json_compact_invalid_json
        PASS [   0.091s] codex-tui text_formatting::tests::test_format_json_compact_empty_object
        PASS [   0.082s] codex-tui text_formatting::tests::test_format_json_compact_nested_object
        PASS [   0.070s] codex-tui text_formatting::tests::test_format_json_compact_primitive_values
        PASS [   0.075s] codex-tui text_formatting::tests::test_format_json_compact_with_whitespace
        PASS [   0.078s] codex-tui text_formatting::tests::test_format_json_compact_simple_object
        PASS [   0.071s] codex-tui text_formatting::tests::test_truncate_emoji
        PASS [   0.075s] codex-tui text_formatting::tests::test_proper_join
        PASS [   0.069s] codex-tui text_formatting::tests::test_truncate_empty_string
        PASS [   0.073s] codex-tui text_formatting::tests::test_truncate_max_graphemes_three_boundary
        PASS [   0.095s] codex-tui text_formatting::tests::test_truncate_max_graphemes_one
        PASS [   0.106s] codex-tui text_formatting::tests::test_truncate_max_graphemes_two
        PASS [   0.071s] codex-tui text_formatting::tests::test_truncate_max_graphemes_zero
        PASS [   0.081s] codex-tui text_formatting::tests::test_truncate_text
        PASS [   0.073s] codex-tui text_formatting::tests::test_truncate_text_exact_length
        LEAK [   0.810s] codex-tui tests::fork_last_filters_latest_session_by_cwd_unless_show_all
        PASS [   0.079s] codex-tui text_formatting::tests::test_truncate_unicode_combining_characters
        PASS [   0.094s] codex-tui text_formatting::tests::test_truncate_text_shorter_than_limit
        PASS [   0.053s] codex-tui text_formatting::tests::test_truncate_very_long_text
        PASS [   0.083s] codex-tui theme_picker::tests::subtitle_falls_back_for_94_column_terminal_side_by_side_layout
        PASS [   0.093s] codex-tui theme_picker::tests::subtitle_falls_back_to_preview_instructions_without_tilde_path
        PASS [   0.069s] codex-tui theme_picker::tests::subtitle_uses_tilde_path_when_codex_home_under_home_directory
        PASS [   0.085s] codex-tui theme_picker::tests::subtitle_falls_back_when_tilde_path_subtitle_is_too_wide
        PASS [   0.091s] codex-tui theme_picker::tests::theme_picker_items_include_search_values_for_preview_mapping
        PASS [   0.084s] codex-tui theme_picker::tests::theme_picker_uses_half_width_with_stacked_fallback_preview
        PASS [   0.078s] codex-tui theme_picker::tests::unavailable_configured_theme_falls_back_to_configured_or_default_selection
        PASS [   0.060s] codex-tui tooltips::tests::announcement_tip_toml_bad_deserialization
        PASS [   0.062s] codex-tui tooltips::tests::announcement_tip_toml_matches_target_os
        LEAK [   1.038s] codex-tui tests::embedded_app_server_supports_thread_start_rpc
        LEAK [   1.008s] codex-tui tests::latest_session_lookup_falls_back_for_rollout_missing_from_state_db
        PASS [   0.068s] codex-tui tooltips::tests::announcement_tip_toml_matches_target_plan_type
        PASS [   0.072s] codex-tui tooltips::tests::announcement_tip_toml_parse_comments
        PASS [   0.079s] codex-tui tooltips::tests::announcement_tip_toml_picks_last_matching
        PASS [  10.135s] codex-tui pets::model::tests::load_builtin_pet_uses_app_catalog_storage
        PASS [   0.273s] codex-tui theme_picker::tests::deleted_preview_code_uses_dim_overlay_like_real_diff_renderer
        PASS [   0.068s] codex-tui tooltips::tests::announcement_tip_toml_picks_no_match
        PASS [   0.060s] codex-tui tooltips::tests::announcement_tip_toml_rejects_unknown_target_plan_type
        PASS [   0.060s] codex-tui tooltips::tests::announcement_tip_toml_rejects_unknown_target_os
        PASS [   0.290s] codex-tui theme_picker::tests::narrow_preview_renders_single_add_and_single_remove_in_four_lines
        PASS [   0.064s] codex-tui tooltips::tests::paid_tooltip_pool_rotates_between_promos
        PASS [   0.077s] codex-tui tooltips::tests::paid_tooltip_pool_skips_fast_when_fast_mode_is_enabled
        PASS [   0.075s] codex-tui tooltips::tests::random_tooltip_is_reproducible_with_seed
        PASS [   0.079s] codex-tui tooltips::tests::random_tooltip_returns_some_tip_when_available
        PASS [   0.075s] codex-tui transcript_reflow::tests::clear_pending_reflow_allows_same_width_to_be_rescheduled
        PASS [   0.082s] codex-tui transcript_reflow::tests::clear_resets_stream_reflow_flags
        PASS [   0.086s] codex-tui transcript_reflow::tests::first_observed_width_marks_reflow_baseline
        PASS [   0.276s] codex-tui theme_picker::tests::wide_preview_renders_all_lines_with_vertical_center_and_left_inset
        PASS [   0.090s] codex-tui transcript_reflow::tests::mark_reflowed_width_records_actual_rebuild_width
        PASS [   0.092s] codex-tui transcript_reflow::tests::mark_reflowed_width_reports_unchanged_width
        PASS [   0.081s] codex-tui transcript_reflow::tests::pending_reflow_target_prevents_repeated_reschedule
        PASS [   0.076s] codex-tui transcript_reflow::tests::reflow_needed_compares_against_actual_rebuild_width
        PASS [   0.075s] codex-tui transcript_reflow::tests::schedule_debounced_postpones_due_existing_reflow
        LEAK [   1.040s] codex-tui tests::lookup_session_target_by_name_uses_backend_title_search
        PASS [   0.078s] codex-tui transcript_reflow::tests::schedule_debounced_postpones_existing_reflow
        PASS [   0.076s] codex-tui transcript_reflow::tests::take_stream_finish_reflow_needed_drains_ran_during_stream
        PASS [   0.075s] codex-tui transcript_reflow::tests::take_stream_finish_reflow_needed_drains_resize_request
        PASS [   0.082s] codex-tui tui::event_stream::tests::draw_and_key_events_yield_both
        PASS [   0.083s] codex-tui tui::event_stream::tests::error_or_eof_ends_stream
        PASS [   0.085s] codex-tui tui::event_stream::tests::key_event_skips_unmapped
        PASS [   0.085s] codex-tui tui::event_stream::tests::lagged_draw_maps_to_draw
        PASS [   0.088s] codex-tui tui::event_stream::tests::resize_event_maps_to_resize
        PASS [   0.086s] codex-tui tui::event_stream::tests::resume_wakes_paused_stream
        PASS [   0.088s] codex-tui tui::event_stream::tests::resume_wakes_pending_stream
        PASS [   0.104s] codex-tui tui::frame_rate_limiter::tests::clamps_to_min_interval_since_last_emit
        PASS [   0.096s] codex-tui tui::frame_rate_limiter::tests::default_does_not_clamp
        PASS [   0.105s] codex-tui tui::frame_requester::tests::test_coalesces_mixed_immediate_and_delayed_requests
        PASS [   0.054s] codex-tui tui::frame_requester::tests::test_schedule_frame_in_triggers_at_delay
        PASS [   0.095s] codex-tui tui::frame_requester::tests::test_limits_draw_notifications_to_120fps
        PASS [   0.106s] codex-tui tui::frame_requester::tests::test_multiple_delayed_requests_coalesce_to_earliest
        PASS [   0.110s] codex-tui tui::frame_requester::tests::test_rate_limit_clamps_early_delayed_requests
        PASS [   0.117s] codex-tui tui::frame_requester::tests::test_rate_limit_does_not_delay_future_draws
        PASS [   0.156s] codex-tui tui::frame_requester::tests::test_coalesces_multiple_requests_into_single_draw
        PASS [   0.115s] codex-tui tui::frame_requester::tests::test_schedule_frame_immediate_triggers_once
        PASS [   0.112s] codex-tui tui::keyboard_modes::tests::disable_modify_other_keys_resets_xterm_keyboard_reporting
        PASS [   0.095s] codex-tui tui::keyboard_modes::tests::enable_modify_other_keys_requests_xterm_keyboard_reporting
        PASS [   0.090s] codex-tui tui::keyboard_modes::tests::keyboard_enhancement_auto_disable_requires_wsl_and_vscode
        PASS [   0.094s] codex-tui tui::keyboard_modes::tests::keyboard_enhancement_auto_disables_for_vscode_in_wsl
        PASS [   0.090s] codex-tui tui::keyboard_modes::tests::keyboard_enhancement_env_flag_overrides_auto_detection
        PASS [   0.085s] codex-tui tui::keyboard_modes::tests::keyboard_enhancement_env_flag_parses_common_values
        PASS [   0.087s] codex-tui tui::keyboard_modes::tests::reset_keyboard_enhancement_flags_clears_all_pushed_levels
        PASS [   0.085s] codex-tui tui::keyboard_modes::tests::tmux_modify_other_keys_only_requests_confirmed_csi_u_format
        PASS [   0.083s] codex-tui tui::keyboard_modes::tests::tmux_session_detection_accepts_tmux_or_tmux_pane
        PASS [   0.081s] codex-tui tui::keyboard_modes::tests::vscode_terminal_detection_uses_linux_and_windows_term_program
        PASS [   0.084s] codex-tui tui::tests::always_notification_condition_emits_when_focused
        PASS [   0.084s] codex-tui tui::tests::first_viewport_change_clears_from_new_viewport_when_old_viewport_is_empty
        PASS [   0.088s] codex-tui tui::tests::unfocused_notification_condition_emits_when_unfocused
        PASS [   0.085s] codex-tui tui::tests::unfocused_notification_condition_is_suppressed_when_focused
        PASS [   0.093s] codex-tui update_action::tests::maps_install_context_to_update_action
        PASS [   0.089s] codex-tui update_action::tests::standalone_update_commands_rerun_latest_installer
        PASS [   0.086s] codex-tui update_versions::tests::extracts_version_from_latest_tag
        PASS [   0.092s] codex-tui update_versions::tests::latest_tag_without_prefix_is_invalid
        PASS [   0.092s] codex-tui update_versions::tests::plain_semver_comparisons_work
        PASS [   0.089s] codex-tui update_versions::tests::prerelease_version_is_not_considered_newer
        PASS [   0.094s] codex-tui update_versions::tests::source_build_version_is_not_checked
        PASS [   0.089s] codex-tui update_versions::tests::whitespace_is_ignored
        PASS [   0.089s] codex-tui voice::tests::convert_pcm16_downmixes_and_resamples_for_model_input
        PASS [   0.090s] codex-tui width::tests::usable_content_width_returns_none_when_reserved_exhausts_width
        PASS [   0.092s] codex-tui width::tests::usable_content_width_u16_matches_usize_variant
        PASS [   0.080s] codex-tui wrapping::tests::adaptive_wrap_line_mixed_line_counts_leading_spaces_before_first_word
        PASS [   0.091s] codex-tui wrapping::tests::adaptive_wrap_line_keeps_long_url_like_token_intact
        PASS [   0.089s] codex-tui wrapping::tests::adaptive_wrap_line_mixed_line_keeps_regular_words_intact
        PASS [   0.087s] codex-tui wrapping::tests::adaptive_wrap_line_mixed_line_resplits_long_token_for_continuation_width
        PASS [   0.088s] codex-tui wrapping::tests::adaptive_wrap_line_mixed_line_wraps_long_non_url_token
        PASS [   0.087s] codex-tui wrapping::tests::adaptive_wrap_line_preserves_default_behavior_for_non_url_tokens
        PASS [   0.088s] codex-tui wrapping::tests::ascii_space_separator_with_no_hyphenation_keeps_url_intact
        PASS [   0.091s] codex-tui wrapping::tests::borrowed_slice_range_rejects_slices_outside_source_text
        PASS [   0.089s] codex-tui wrapping::tests::break_words_false_allows_overflow_for_long_word
        PASS [   0.090s] codex-tui wrapping::tests::empty_initial_indent_subsequent_spaces
        PASS [   0.090s] codex-tui wrapping::tests::empty_input_yields_single_empty_line
        PASS [   0.092s] codex-tui wrapping::tests::hyphen_splitter_breaks_at_hyphen
        PASS [   0.089s] codex-tui wrapping::tests::indent_consumes_width_leaving_one_char_space
        PASS [   0.094s] codex-tui wrapping::tests::leading_spaces_preserved_on_first_line
        PASS [   0.093s] codex-tui wrapping::tests::line_contains_url_like_checks_across_spans
        PASS [   0.094s] codex-tui wrapping::tests::line_has_mixed_url_and_non_url_tokens_detects_prose_plus_url
        PASS [   0.090s] codex-tui wrapping::tests::line_has_mixed_url_and_non_url_tokens_ignores_ordered_list_marker
        PASS [   0.091s] codex-tui wrapping::tests::line_has_mixed_url_and_non_url_tokens_ignores_pipe_prefix
        PASS [   0.094s] codex-tui wrapping::tests::line_height_counts_double_width_emoji
        PASS [   0.091s] codex-tui wrapping::tests::map_owned_wrapped_line_to_range_indent_coincides_with_source
        PASS [   0.094s] codex-tui wrapping::tests::map_owned_wrapped_line_to_range_recovers_on_non_prefix_mismatch
        PASS [   0.090s] codex-tui wrapping::tests::map_owned_wrapped_line_to_range_repro_overconsumes_repeated_prefix_patterns
        PASS [   0.092s] codex-tui wrapping::tests::multiple_spaces_between_words_dont_start_next_line_with_spaces
        PASS [   0.090s] codex-tui wrapping::tests::simple_styled_wrap_preserves_styles
        PASS [   0.088s] codex-tui wrapping::tests::simple_unstyled_wrap_narrow_width
        PASS [   0.088s] codex-tui wrapping::tests::styled_split_within_span_preserves_style
        PASS [   0.086s] codex-tui wrapping::tests::text_contains_url_like_accepts_custom_scheme_with_separator
        PASS [   0.098s] codex-tui wrapping::tests::text_contains_url_like_matches_expected_tokens
        PASS [   0.090s] codex-tui wrapping::tests::text_contains_url_like_rejects_invalid_ports
        PASS [   0.090s] codex-tui wrapping::tests::text_contains_url_like_rejects_non_urls
        PASS [   0.092s] codex-tui wrapping::tests::trivial_unstyled_no_indents_wide_width
        PASS [   0.093s] codex-tui wrapping::tests::wide_unicode_wraps_by_display_width
        PASS [   0.094s] codex-tui wrapping::tests::with_initial_and_subsequent_indents
        PASS [   0.092s] codex-tui wrapping::tests::word_wrap_does_not_split_words_simple_english
        PASS [   0.099s] codex-tui wrapping::tests::wrap_lines_accepts_borrowed_iterators
        PASS [   0.094s] codex-tui wrapping::tests::wrap_lines_applies_initial_indent_only_once
        PASS [   0.101s] codex-tui wrapping::tests::wrap_lines_accepts_str_slices
        PASS [   0.095s] codex-tui wrapping::tests::wrap_lines_borrowed_applies_initial_indent_only_once
        PASS [   0.090s] codex-tui wrapping::tests::wrap_lines_borrowed_without_indents_is_concat_of_single_wraps
        PASS [   0.095s] codex-tui wrapping::tests::wrap_lines_without_indents_is_concat_of_single_wraps
        PASS [   0.093s] codex-tui wrapping::tests::wrap_ranges_indent_prefix_coincides_with_source_char
        PASS [   0.097s] codex-tui wrapping::tests::wrap_ranges_recovers_with_non_space_indents
        PASS [   0.099s] codex-tui wrapping::tests::wrap_ranges_trim_handles_owned_lines_with_penalty_char
        PASS [   0.109s] codex-tui::all suite::status_indicator::ansi_escape_line_strips_escape_sequences
        PASS [   0.104s] codex-tui::all suite::vt100_history::basic_insertion_no_wrap
        PASS [   0.092s] codex-tui::all suite::vt100_history::cursor_restoration
        PASS [   0.081s] codex-tui::all suite::vt100_history::em_dash_and_space_word_wrap
        PASS [   0.084s] codex-tui::all suite::vt100_history::emoji_and_cjk
        PASS [   0.076s] codex-tui::all suite::vt100_history::long_token_wraps
        PASS [   0.082s] codex-tui::all suite::vt100_history::mixed_ansi_spans
        PASS [   0.075s] codex-tui::all suite::vt100_history::word_wrap_no_mid_word_split
        PASS [   0.071s] codex-tui::all suite::vt100_live_commit::live_001_commit_on_overflow
        PASS [   0.254s] codex-tui::manager_dependency_regression tui_runtime_source_does_not_depend_on_manager_escape_hatches
        PASS [  18.120s] codex-tui motion::tests::animation_primitives_are_only_used_by_motion_module
------------
     Summary [  65.896s] 2796 tests run: 2780 passed (40 leaky), 16 failed, 9 skipped
  TRY 2 FAIL [   0.961s] codex-tui app::history_ui::tests::desktop_thread_opened_history_snapshot
  TRY 2 FAIL [   0.883s] codex-tui app::startup_prompts::tests::repeated_active_skill_load_warning_renders_once
  TRY 2 FAIL [   0.827s] codex-tui app_backtrack::tests::backtrack_unavailable_info_message_snapshot
  TRY 2 FAIL [   0.073s] codex-tui bottom_pane::chat_composer::tests::slash_tab_completion_moves_cursor_to_end
  TRY 2 FAIL [   0.229s] codex-tui chatwidget::tests::permissions::approvals_popup_navigation_skips_disabled
  TRY 2 FAIL [   0.943s] codex-tui chatwidget::tests::permissions::approvals_selection_popup_snapshot
  TRY 2 FAIL [   0.111s] codex-tui chatwidget::tests::permissions::permissions_full_access_history_cell_emitted_only_after_confirmation
  TRY 2 FAIL [   0.886s] codex-tui chatwidget::tests::permissions::permissions_selection_history_snapshot_after_mode_switch
  TRY 2 FAIL [   0.916s] codex-tui chatwidget::tests::permissions::permissions_selection_history_snapshot_full_access_to_default
  TRY 2 FAIL [   0.191s] codex-tui chatwidget::tests::permissions::profile_permissions_full_access_opens_confirmation
  TRY 2 FAIL [   0.895s] codex-tui chatwidget::tests::review_mode::interrupted_turn_pending_steers_message_snapshot
  TRY 2 FAIL [   1.054s] codex-tui chatwidget::tests::status_and_layout::workspace_owner_credits_nudge_completion_renders_feedback
  TRY 2 FAIL [   1.032s] codex-tui chatwidget::tests::status_and_layout::workspace_owner_usage_limit_nudge_completion_renders_feedback
  TRY 2 FAIL [   0.106s] codex-tui status::helpers::tests::compose_agents_summary_includes_global_agents_path
  TRY 2 FAIL [   0.078s] codex-tui status::helpers::tests::compose_agents_summary_names_global_agents_override
  TRY 2 FAIL [   0.067s] codex-tui status::helpers::tests::compose_agents_summary_orders_global_before_project_agents
error: test run failed
error: recipe `test` failed on line 69 with exit code 1

(base) C:\CodexLab\codex\codex-rs>

```