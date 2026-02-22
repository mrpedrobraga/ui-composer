use ui_composer::prelude::*;

fn main() {
    UIComposer::run(Window().with_decorations(false))
}

fn App() -> impl UI {
    let current_directory: State<PathBuf> = State::new(None);
    let open_items = StateVec::new();

    let sidebar = FileExplorer(current_directory.view());
    let main_view = MainView(open_items);

    flex(list![item(sidebar), item(main_view).with_grow(1.0)])
}

fn FileExplorer(cwd: impl Signal<Item = PathBuf>) -> impl UI {
    let header = Label("Files").with_align(Align::Center);

    let dir_view = cwd.map(|cwd| {
        let directory_watcher = DirectoryState::of(&cwd);

        directory_watcher
            .entries()
            .map_each(DirectoryEntryView)
            .map(column);
    });

    let drives_view = FileSystemState::new();
    drives_view.drives().map(|drive| todo!());

    flex(list![
        item(header),
        item(dir_view).with_grow(1.0),
        item(drives_view)
    ])
}

fn MainView(
    current_tab: impl Signal<Item = Option<usize>>,
    open_items: impl SignalVec<OpenItem>,
) -> impl UI {
    let item_views = open_items.map(|| {});

    TabContainer().with_current_tab_state(current_tab)
}
