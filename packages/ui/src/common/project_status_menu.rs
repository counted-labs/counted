use shared::ProjectStatus;

/// Every status change offered for a project, as (current status, target status, sheet key). The
/// list page's row sheet and the details page's sheet render this same set.
const TRANSITIONS: &[(ProjectStatus, ProjectStatus, &str)] = &[
    (ProjectStatus::Ongoing, ProjectStatus::Closed, "project-sheet-close"),
    (ProjectStatus::Ongoing, ProjectStatus::Archived, "project-sheet-archive"),
    (ProjectStatus::Closed, ProjectStatus::Ongoing, "project-sheet-reopen"),
    (ProjectStatus::Closed, ProjectStatus::Archived, "project-sheet-archive"),
    (ProjectStatus::Archived, ProjectStatus::Ongoing, "project-sheet-unarchive"),
];

/// The transitions out of `status`, as (target status, sheet key).
pub fn status_transitions(
    status: &ProjectStatus,
) -> impl Iterator<Item = (ProjectStatus, &'static str)> + '_ {
    TRANSITIONS
        .iter()
        .filter(move |(from, ..)| from == status)
        .map(|(_, to, key)| (to.clone(), *key))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `tid!` here is keyed off the table, not a literal, so the crate-wide scanner in `i18n` does
    /// not see these. Without this a typo would ship as a raw key in the sheet.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn every_transition_label_is_a_known_message() {
        let known = crate::i18n::locale_ids(crate::i18n::FALLBACK);
        for (_, _, key) in TRANSITIONS {
            assert!(known.contains(*key), "{key} is not defined in the fallback locale");
        }
    }

    fn targets(status: ProjectStatus) -> Vec<ProjectStatus> {
        status_transitions(&status).map(|(to, _)| to).collect()
    }

    #[test]
    fn ongoing_offers_close_then_archive() {
        assert_eq!(targets(ProjectStatus::Ongoing), vec![ProjectStatus::Closed, ProjectStatus::Archived]);
    }

    #[test]
    fn closed_offers_reopen_then_archive() {
        assert_eq!(targets(ProjectStatus::Closed), vec![ProjectStatus::Ongoing, ProjectStatus::Archived]);
    }

    #[test]
    fn archived_offers_unarchive_only() {
        assert_eq!(targets(ProjectStatus::Archived), vec![ProjectStatus::Ongoing]);
    }
}
