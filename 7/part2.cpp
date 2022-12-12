#include <fstream>
#include <string>
#include <iostream>
#include <string_view>
#include <ranges>
#include <iterator>
#include <vector>
#include <charconv>

namespace views = std::views;
namespace rng = std::ranges;

class Dirs {
	struct Node{
		std::vector<Node> children {0};
		Node* parent = nullptr;
		int weight = 0;
		std::string name;
		//returns the index of inserted child
		int insert(Node&& inserted){
			std::string_view new_name = inserted.name;
			for (int idx = 0; idx < children.size(); ++idx) {
				if (children[idx].name == new_name) {
					return idx;
				}
			}
			children.push_back(inserted);
			return children.size() - 1;
		}
		void visit_all_nodes(int& smallest,const int req) {
			for (auto& child : children) {
				if (child.weight >= req && smallest >= child.weight) {
					smallest = child.weight;
				}
				child.visit_all_nodes(smallest,req);
			}
		}
		void addWeight(int weight) {
			if (parent != nullptr) parent->addWeight(weight);
			this->weight += weight;
		}
	};

	Node* curr = nullptr;

	auto cd(const std::string_view name) {
		using std::operator""sv;
		if (name.compare(".."sv) == 0) {curr = curr->parent;return; }
		auto inserted = curr->insert(Node { .name = std::string{name}});
		Node* parent = curr;
		curr = &curr->children.at(inserted);
		curr->parent = parent;
	}

	auto ls(const std::string_view cmd) {
		if (cmd.at(0) == '$') { return false; }
		using std::operator""sv;
		auto words = cmd | rng::views::split(' ');
		auto it = rng::begin(words);
		std::string_view type{(*it).begin(),(*it).end()};

		if (type.find_first_not_of("0123456789"sv) == std::string_view::npos) {
			int weight = 0;
			std::from_chars(type.data(), type.data() + type.size(), weight);
			curr->addWeight(weight);
		}
		else {
			std::string dir_name;
			for (const auto& dir : words | views::drop(1)) { dir_name = std::string_view{ dir.begin(),dir.end() }; }
			curr->insert(Node{.parent = curr, .name = dir_name });
		}
		return true;
	}

	auto parse_command(const std::string_view& command){
		using std::operator""sv;
		if (command.at(0) == '$') {
			auto words = command | rng::views::split(' ') | views::drop(1);
			auto it = rng::begin(words);
			std::string_view cmd{(*it).begin(),(*it).end()};
			++it;
			if (cmd.compare("cd"sv) == 0) {
				std::string_view name {(*it).begin(),(*it).end()};
				cd(name);
				return false;
			}
			else if (cmd.compare("ls"sv) == 0) {
				return true;
			}
		}
		return false;
	}
public:
	auto run(const std::string& file_name){
		std::ifstream file{file_name};
		Node base {.name = "/"};
		curr = &base;
		bool ls_flag = false;

		for(std::string line;std::getline(file,line);){
			if (ls_flag) {
				if (ls(line)) continue;
				ls_flag = false;
			}
			ls_flag = parse_command(line);
		}
		int unused = 70000000 - base.weight;
		int req = 30000000 - unused;
		int smallest = 700000000;
		base.visit_all_nodes(smallest,req);
		std::cout << smallest;
	}
};

int main() {
	Dirs dir;
	dir.run("./input.txt");
}
