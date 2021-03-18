use super::changes::{Changes, DirectoryCreation, FileCreation};

pub struct GetChangesForNewProject {
    project_path: String,
    project_name: String,
    changes: Vec<Changes>,
}

impl GetChangesForNewProject {
    pub fn new(project_path: String, project_name: String) -> Self {
        Self {
            project_path,
            project_name,
            changes: vec![],
        }
    }

    pub fn run(&mut self) -> Vec<Changes> {
        self.create_root_directory();
        self.create_clients_directory();
        self.create_contracts_directory();
        self.create_notebooks_directory();
        self.create_scripts_directory();
        self.create_environments_directory();
        self.create_tests_directory();

        self.create_clarinet_toml();
        // self.create_environment_mainnet_toml();
        // self.create_environment_testnet_toml();
        self.create_environment_dev_toml();
        self.changes.clone()
    }

    fn create_root_directory(&mut self) {
        let dir = format!("{}/{}", self.project_path, self.project_name);
        let change = DirectoryCreation {
            comment: format!("Creating directory {}", self.project_name),
            name: self.project_name.clone(),
            path: dir,
        };
        self.changes.push(Changes::AddDirectory(change));
    }

    fn create_clients_directory(&mut self) {
        self.changes
            .push(self.get_changes_for_new_root_dir(format!("clients")));
    }

    fn create_contracts_directory(&mut self) {
        self.changes
            .push(self.get_changes_for_new_root_dir(format!("contracts")));
    }

    fn create_notebooks_directory(&mut self) {
        self.changes
            .push(self.get_changes_for_new_root_dir(format!("notebooks")));
    }

    fn create_scripts_directory(&mut self) {
        self.changes
            .push(self.get_changes_for_new_root_dir(format!("scripts")));
    }

    fn create_environments_directory(&mut self) {
        self.changes
            .push(self.get_changes_for_new_root_dir(format!("environments")));
    }

    fn create_tests_directory(&mut self) {
        self.changes
            .push(self.get_changes_for_new_root_dir(format!("tests")));
    }

    fn create_clarinet_toml(&mut self) {
        let content = format!(
            r#"
[project]
name = "{}"

[contracts]

[notebooks]
"#,
            self.project_name
        );
        let name = format!("Clarinet.toml");
        let path = format!("{}/{}/{}", self.project_path, self.project_name, name);
        let change = FileCreation {
            comment: format!("Creating file {}/{}", self.project_name, name),
            name,
            content,
            path,
        };
        self.changes.push(Changes::AddFile(change));
    }

    #[allow(dead_code)]
    fn create_environment_mainnet_toml(&mut self) {
        let content = format!(
            r#"[network]
name = "mainnet"
node_rpc_address = "http://stacks-node-api.blockstack.org:20443"
"#
        );
        let name = format!("Mainnet.toml");
        let path = format!(
            "{}/{}/environments/{}",
            self.project_path, self.project_name, name
        );
        let change = FileCreation {
            comment: format!("Creating file {}/environments/{}", self.project_name, name),
            name,
            content,
            path,
        };
        self.changes.push(Changes::AddFile(change));
    }

    #[allow(dead_code)]
    fn create_environment_testnet_toml(&mut self) {
        let content = format!(
            r#"[network]
name = "testnet"
node_rpc_address = "http://xenon.blockstack.org:20443"
"#
        );
        let name = format!("Testnet.toml");
        let path = format!(
            "{}/{}/environments/{}",
            self.project_path, self.project_name, name
        );
        let change = FileCreation {
            comment: format!("Creating file {}/environments/{}", self.project_name, name),
            name,
            content,
            path,
        };
        self.changes.push(Changes::AddFile(change));
    }

    fn create_environment_dev_toml(&mut self) {
        let content = format!(
            r#"[network]
name = "Development"

[accounts.wallet_1]
mnemonic = "dance news bachelor pink hammer clerk solve lake mushroom warm draw cousin forest shock believe smoke lift spin laundry couch gloom hold hurry decline"
balance = 1_000_000

[accounts.wallet_2]
mnemonic = "apology together shy taxi glare struggle hip camp engage lion possible during squeeze hen exotic marriage misery kiwi once quiz enough exhibit immense tooth"
balance = 1_000_000

[accounts.wallet_3]
mnemonic = "replace swing shove congress smoke banana tired term blanket nominee leave club myself swing egg virus answer bulk useful start decrease family energy february"
balance = 1_000_000

[accounts.wallet_4]
mnemonic = "fetch outside black test wash cover just actual execute nice door want airport betray quantum stamp fish act pen trust portion fatigue scissors vague"
balance = 1_000_000

[accounts.wallet_5]
mnemonic = "east load echo merit ignore hip tag obvious truly adjust smart panther deer aisle north hotel process frown lock property catch bless notice topple"
balance = 1_000_000

[accounts.wallet_6]
mnemonic = "pulp when detect fun unaware reduce promote tank success lecture cool cheese object amazing hunt plug wing month hello tunnel detect connect floor brush"
balance = 1_000_000

[accounts.wallet_7]
mnemonic = "glide clown kitchen picnic basket hidden asset beyond kid plug carbon talent drama wet pet rhythm hero nest purity baby bicycle ghost sponsor dragon"
balance = 1_000_000

[accounts.wallet_8]
mnemonic = "antenna bitter find rely gadget father exact excuse cross easy elbow alcohol injury loud silk bird crime cabbage winter fit wide screen update october"
balance = 1_000_000

[accounts.wallet_9]
mnemonic = "market ocean tortoise venue vivid coach machine category conduct enable insect jump fog file test core book chaos crucial burst version curious prosper fever"
balance = 1_000_000

[accounts.wallet_10]
mnemonic = "recall also where motion brave sketch grass belt attract vapor whip ski apple force pear width slot ocean slide piece rail excite learn gain"
balance = 1_000_000
"#
        );
        let name = format!("Development.toml");
        let path = format!(
            "{}/{}/environments/{}",
            self.project_path, self.project_name, name
        );
        let change = FileCreation {
            comment: format!("Creating file {}/environments/{}", self.project_name, name),
            name,
            content,
            path,
        };
        self.changes.push(Changes::AddFile(change));
    }

    fn get_changes_for_new_root_dir(&self, name: String) -> Changes {
        let dir = format!("{}/{}", self.project_name, name);
        let change = DirectoryCreation {
            comment: format!("Creating directory {}/{}", self.project_name, name),
            name,
            path: dir,
        };
        Changes::AddDirectory(change)
    }
}
