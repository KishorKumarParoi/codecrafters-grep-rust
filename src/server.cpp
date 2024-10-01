// #include <algorithm>
// #include <charconv>
// #include <cstdint>
// #include <iostream>
// #include <limits>
// #include <string>
// #include <unordered_map>
// #include <utility>
// namespace
// {
// 	bool isDigit(char chr)
// 	{
// 		return std::isdigit(chr) != 0;
// 	}
// 	bool isWord(char chr)
// 	{
// 		return std::isalnum(chr) != 0;
// 	}
// 	bool isPrint(char chr)
// {
// 	return std::isprint(chr) != 0;
// }
// uint16_t toU16(std::string_view value)
// {
// 	uint16_t result = 0;
// 	const auto status = std::from_chars(value.data(), value.data() + value.size(), result);
// 	return status.ec == std::errc() ? result : 0;
// }
// enum class PatternType : uint8_t
// {
// 	Print,
// 	PositiveClass,
// 	NegativeClass,
// 	Digit,
// 	Word,
// 	BeginAnchor,
// 	EndAnchor,
// 	Any,
// 	Group,
// 	Backreference,
// };
// class PatternResult
// {
// public:
// 	PatternResult(PatternType type, std::string_view value, std::string_view remainder) :
// 	    type(type), value(value), remainder(remainder)
// 	{
// 		if(!this->remainder.empty())
// 		{
// 			switch(this->remainder.front())
// 			{
// 			case '+':
// 				max = std::numeric_limits<uint16_t>::max();
// 				this->remainder.remove_prefix(1);
// 				break;
// 			case '?':
// 				min = 0;
// 				this->remainder.remove_prefix(1);
// 				break;
// 			}
// 		}
// 	}
// 	PatternType type;
// 	uint16_t min = 1;
// 	uint16_t max = 1;
// 	std::string_view value;
// 	std::string_view remainder;
// };
// PatternResult getPattern(std::string_view pattern)
// {
// 	if(pattern.size() >= 2 && pattern[0] == '\\')
// 	{
// 		switch(pattern[1])
// 		{
// 		case 'd':
// 			return {PatternType::Digit, {}, pattern.substr(2)};
// 		case 'w':
// 			return {PatternType::Word, {}, pattern.substr(2)};
// 		}
// 		if(isDigit(pattern[1]))
// 		{
// 			const auto digits = std::min(pattern.find_first_not_of("0123456789", 1), pattern.size()) - 1;
// 			return {PatternType::Backreference, pattern.substr(1, digits), pattern.substr(digits + 1)};
// 		}
// 	}
// 	if(pattern.size() >= 2 && pattern[0] == '[')
// 	{
// 		if(const size_t end = pattern.find(']'); end != std::string_view::npos)
// 		{
// 			if(pattern[1] == '^')
// 			{
// 				return {PatternType::NegativeClass, pattern.substr(2, end - 2), pattern.substr(end + 1)};
// 			}
// 			return {PatternType::PositiveClass, pattern.substr(1, end - 1), pattern.substr(end + 1)};
// 		}
// 	}
// 	if(pattern.size() >= 3 && pattern[0] == '(')
// 	{
// 		if(const size_t end = pattern.find(')'); end != std::string_view::npos)
// 		{
// 			return {PatternType::Group, pattern.substr(1, end - 1), pattern.substr(end + 1)};
// 		}
// 	}
// 	if(!pattern.empty())
// 	{
// 		switch(pattern[0])
// 		{
// 		case '^':
// 			return {PatternType::BeginAnchor, {}, pattern.substr(1)};
// 		case '$':
// 			return {PatternType::EndAnchor, {}, pattern.substr(1)};
// 		case '.':
// 			return {PatternType::Any, {}, pattern.substr(1)};
// 		}
// 		if(isPrint(pattern[0]))
// 		{
// 			return {PatternType::Print, pattern.substr(0, 1), pattern.substr(1)};
// 		}
// 	}
// 	throw std::runtime_error(std::string{"Unhandled pattern "}.append(pattern));
// }
// bool matchSingle(char chrToCheck, const PatternResult& subPattern)
// {
// 	switch(subPattern.type)
// 	{
// 	case PatternType::Print:
// 		return chrToCheck == subPattern.value.front();
// 	case PatternType::Any:
// 		return true;
// 	case PatternType::Digit:
// 		return isDigit(chrToCheck);
// 	case PatternType::Word:
// 		return isWord(chrToCheck);
// 	case PatternType::PositiveClass:
// 		return subPattern.value.contains(chrToCheck);
// 	case PatternType::NegativeClass:
// 		return !subPattern.value.contains(chrToCheck);
// 	case PatternType::BeginAnchor:
// 	case PatternType::EndAnchor:
// 		throw std::runtime_error("Unexpected anchor");
// 	case PatternType::Group:
// 		throw std::runtime_error("Unexpected group");
// 	case PatternType::Backreference:
// 		throw std::runtime_error("Unexpected backreference");
// 	}
// 	std::unreachable();
// #include <vector>
// #include <algorithm>
// #include <cctype>
// #include <stdexcept>
// size_t find_char_at_same_level(const std::string& pattern, const char c, const size_t start) {
//     size_t count = 0;
//     for (size_t i = start; i < pattern.length(); ++i) {
//         if (pattern[i] == '(') ++count;
//         if (pattern[i] == c && count == 0) return i;
//         if (pattern[i] == ')') --count;
//     }
//     return std::string::npos;
// }
// uint16_t matchGroup(std::string_view inputLine, const PatternResult& groupPattern,
// 	std::unordered_map<uint16_t, std::string_view>& backReferences);
// uint16_t matchBackreference(
// 	std::string_view inputLine, std::string_view value, std::unordered_map<uint16_t, std::string_view>& backReferences);
// uint16_t matchRange(std::string_view inputLine, const PatternResult& subPattern,
// 	std::unordered_map<uint16_t, std::string_view>& backReferences)
// {
// 	switch(subPattern.type)
// 	{
// 		case PatternType::Group:
// 			return matchGroup(inputLine, subPattern, backReferences);
// 		case PatternType::Backreference:
// 			return matchBackreference(inputLine, subPattern.value, backReferences);
// 		default:;
// 	}
// 	const size_t max = std::min<size_t>(inputLine.size(), subPattern.max);
// 	uint16_t count = 0;
// 	for(; count < max && matchSingle(inputLine[count], subPattern); ++count)
// 	{
// 	}
// 	return count;
// std::vector<std::string> decompose_pattern(const std::string& pattern) {
//     std::vector<std::string> res;
//     const size_t n = pattern.length();
//     const std::string quantifiers("+?");
//     size_t i = 0;
//     while (i < n) {
//         size_t matched = 0;
//         if (pattern[i] == '\\' && i + 1 < n) {
//             if (i + 2 < n && quantifiers.find(pattern[i + 2]) != std::string::npos) {
//                 matched = 3;
//             } else {
//                 matched = 2;
//             }
//         } else if (pattern[i] == '(' && i + 2 < n) {
//             matched = find_char_at_same_level(pattern, ')', i + 1) - i + 1;
//         } else if (pattern[i] == '[' && i + 2 < n) {
//             const size_t k = pattern.find_first_of(']', i + 1);
//             if (k + 1 < n && quantifiers.find(pattern[k + 1]) != std::string::npos) {
//                 matched = k - i + 2;
//             } else {
//                 matched = k - i + 1;
//             }
//         } else {
//             if (i + 1 < n && quantifiers.find(pattern[i + 1]) != std::string::npos) {
//                 matched = 2;
//             } else {
//                 matched = 1;
//             }
//         }
//         res.push_back(pattern.substr(i, matched));
//         i += matched;
//     }
//     return res;
// }
// std::string_view matchPatternInternal(std::string_view inputLine, std::string_view rawPattern,
// 	std::unordered_map<uint16_t, std::string_view>& backReferences)
// {
// 	const std::string_view original = inputLine;
// 	while(!inputLine.empty() && !rawPattern.empty())
// 	{
// 		const PatternResult subPattern = getPattern(rawPattern);
// 		const uint16_t count = matchRange(inputLine, subPattern, backReferences);
// 		if(count < subPattern.min)
// 		{
// 			return {};
// 		}
// 		inputLine.remove_prefix(count);
// 		rawPattern = subPattern.remainder;
// 	}
// 	if(rawPattern.empty() || getPattern(rawPattern).type == PatternType::EndAnchor)
// 	{
// 		return original.substr(0, original.size() - inputLine.size());
// 	}
// 	return {};
// bool char_matches_pattern(const char c, const std::string& pattern) {
//     if (pattern == ".") {
//         return true;
//     } else if (pattern.length() == 1) {
//         return c == pattern[0];
//     } else if (pattern == "\\d") {
//         return std::isdigit(static_cast<unsigned char>(c));
//     } else if (pattern == "\\w") {
//         return std::isalnum(static_cast<unsigned char>(c));
//     } else if (pattern.size() >= 3 && pattern.substr(0, 2) == "[^" && pattern.back() == ']') {
//         const std::string chars(pattern.cbegin() + 2, pattern.cend() - 1);
//         return chars.find(c) == std::string::npos;
//     } else if (pattern.front() == '[' && pattern.back() == ']') {
//         const std::string chars(pattern.cbegin() + 1, pattern.cend() - 1);
//         return chars.find(c) != std::string::npos;
//     } else {
//         throw std::runtime_error("Unhandled pattern " + pattern);
//     }
// }
// std::string_view matchPattern(std::string_view inputLine, std::string_view rawPattern)
// {
// 	std::unordered_map<uint16_t, std::string_view> backReferences;
// 	const PatternResult subPattern = getPattern(rawPattern);
// 	if(subPattern.type == PatternType::BeginAnchor)
// 	{
// 		return matchPatternInternal(inputLine, subPattern.remainder, backReferences);
// 	}
// 	while(!inputLine.empty())
// 	{
// 		uint16_t count = matchRange(inputLine, subPattern, backReferences);
// 		while(!inputLine.empty() && count < subPattern.min)
// 		{
// 			inputLine.remove_prefix(1);
// 			count = matchRange(inputLine, subPattern, backReferences);
// 		}
// 		if(inputLine.empty())
// 		{
// 			return {};
// 		}
// 		rawPattern = subPattern.remainder;
// 		if(rawPattern.empty())
// 		{
// 			return inputLine.substr(0, count);
// 		}
// 		if(const auto matched = matchPatternInternal(inputLine.substr(count), rawPattern, backReferences);
// 	!matched.empty())
// 		{
// 			return inputLine.substr(0, count + matched.size());
// 		}
// 		inputLine.remove_prefix(count);
// 	}
// 	return {};
// void save(std::vector<std::string>& captures, std::string&& s) {
//     for (auto it = captures.rbegin(); it != captures.rend(); ++it) {
//         if (it->empty()) {
//             *it = std::move(s);
//             break;
//         }
//     }
// }
// uint16_t matchGroup(std::string_view inputLine, const PatternResult& groupPattern,
// 	std::unordered_map<uint16_t, std::string_view>& backReferences)
// {
// 	std::string_view rawPattern = groupPattern.value;
// 	while(!rawPattern.empty())
// 	{
// 		const size_t subLength = std::min(rawPattern.find('|'), rawPattern.size());
// 		if(const auto matched = matchPatternInternal(inputLine, rawPattern.substr(0, subLength), backReferences);
// 	!matched.empty())
// 		{backReferences[backReferences.size() + 1] = matched;
// 			return matched.size();
// 		}
// 		rawPattern.remove_prefix(std::min(subLength + 1, rawPattern.size()));
// 	}
// 	return 0;
// size_t match_pattern_at_beginning(const std::string& input_line, const std::vector<std::string>& patterns,
//                                   const std::string* next_pattern, std::vector<std::string>& captures) {
//     const size_t n = patterns.size();
//     const size_t m = input_line.length();
//     size_t pos = 0;
//     for (size_t i = 0; i < n; ++i) {
//         if (patterns[i] == "$") {
//             return pos == m ? m : 0;
//         }
//         if (patterns[i].front() == '(' && patterns[i].back() == ')') {
//             captures.emplace_back("");
//             const auto p = find_char_at_same_level(patterns[i], '|', 1);
//             if (p == std::string::npos) {
//                 const auto pattern = decompose_pattern(patterns[i].substr(1, patterns[i].length() - 2));
//                 const std::string* next = i < n - 1 ? &patterns[i + 1] : nullptr;
//                 const auto sz = match_pattern_at_beginning(input_line.substr(pos), pattern, next, captures);
//                 if (sz == 0) {
//                     return 0;
//                 } else {
//                     save(captures, input_line.substr(pos, sz));
//                     pos += sz;
//                 }
//             } else {
//                 const auto pattern1 = decompose_pattern(patterns[i].substr(1, p - 1));
//                 const auto pattern2 = decompose_pattern(patterns[i].substr(p + 1, patterns[i].length() - p - 2));
//                 const auto sz1 = match_pattern_at_beginning(input_line.substr(pos), pattern1, nullptr, captures);
//                 const auto sz2 = match_pattern_at_beginning(input_line.substr(pos), pattern2, nullptr, captures);
//                 if (sz1 > 0) {
//                     save(captures, input_line.substr(pos, sz1));
//                     pos += sz1;
//                 } else if (sz2 > 0) {
//                     save(captures, input_line.substr(pos, sz2));
//                     pos += sz2;
//                 } else {
//                     return 0;
//                 }
//             }
//         } else if (patterns[i].back() == '+') {
//             const std::string pattern = patterns[i].substr(0, patterns[i].length() - 1);
//             if (pos >= m || !char_matches_pattern(input_line[pos], pattern)) {
//                 return 0;
//             }
//             ++pos;
//             while (pos < m && char_matches_pattern(input_line[pos], pattern)) {
//                 std::vector<std::string> tmp;
//                 std::vector<std::string> to_check;
//                 if (next_pattern != nullptr) to_check.push_back(*next_pattern);
//                 else if (i < n - 1) to_check.push_back(patterns[i]);
//                 if (pattern.substr(0, 2) == "[^" && match_pattern_at_beginning(input_line.substr(pos), to_check, nullptr, tmp)) {
//                     break;
//                 }
//                 ++pos;
//             }
//         } else if (patterns[i].back() == '?') {
//             const std::string pattern = patterns[i].substr(0, patterns[i].length() - 1);
//             if (pos < m && char_matches_pattern(input_line[pos], pattern)) {
//                 ++pos;
//             }
//         } else if (patterns[i].length() == 2 && patterns[i][0] == '\\' && std::isdigit(static_cast<unsigned char>(patterns[i][1]))) {
//             size_t k = std::stoul(patterns[i].substr(1));
//             if (captures.size() < k) {
//                 throw std::runtime_error("Invalid backreference " + patterns[i]);
//             }
//             const auto& group = captures[k - 1];
//             if (input_line.substr(pos, group.length()) != group) {
//                 return 0;
//             }
//             pos += group.length();
//         } else if (pos >= m || !char_matches_pattern(input_line[pos], patterns[i])) {
//             return 0;
//         } else {
//             ++pos;
//         }
//     }
//     return pos;
// }
// uint16_t matchBackreference(
// 	std::string_view inputLine, std::string_view value, std::unordered_map<uint16_t, std::string_view>& backReferences)
// {
// 	const std::string_view rawPattern = backReferences.at(toU16(value));
// 	if(const auto matched = matchPatternInternal(inputLine, rawPattern, backReferences); !matched.empty())
// 	{
// 		return matched.size();
// 	}
// 	return 0;
// bool match_pattern_at_beginning(const std::string& input_line, const std::vector<std::string>& patterns) {
//     std::vector<std::string> captures;
//     return match_pattern_at_beginning(input_line, patterns, nullptr, captures) > 0;
// }
// bool match_pattern(const std::string& input_line, const std::string& pattern) {
//     const auto patterns = decompose_pattern(pattern);
//     if (patterns.empty()) throw std::runtime_error("Unhandled pattern " + pattern);
//     if (patterns[0] == "^") {
//         return match_pattern_at_beginning(input_line, {patterns.cbegin() + 1, patterns.cend()});
//     }
//     for (size_t i = 0; i < input_line.length(); ++i) {
//         if (match_pattern_at_beginning(input_line.substr(i), patterns)) {
//             return true;
//         }
//     }
//     return false;
// }
// int main(int argc, char* argv[])
// {
// 	// Flush after every std::cout / std::cerr
// 	std::cout << std::unitbuf;
// 	std::cerr << std::unitbuf;
// 	if(argc != 3)
// 	{
// 		std::cerr << "Expected two arguments\n";
// 		return 1;
// 	}
// 	const std::string flag = argv[1];
// 	const std::string pattern = argv[2];
// 	if(flag != "-E")
// 	{
// 		std::cerr << "Expected first argument to be '-E'\n";
// 		return 1;
// 	}
// 	std::string inputLine;
// 	std::getline(std::cin, inputLine);
// 	try
// 	{
// 		return matchPattern(inputLine, pattern).empty() ? 1 : 0;
// 	}
// 	catch(const std::runtime_error& e)
// 	{
// 		std::cout << e.what() << '\n';
// 		return 1;
// 	}
// int main(int argc, char* argv[]) {
//     std::cout << std::unitbuf;
//     std::cerr << std::unitbuf;
//     if (argc != 3) {
//         std::cerr << "Expected two arguments" << std::endl;
//         return 1;
//     }
//     std::string flag = argv[1];
//     std::string pattern = argv[2];
//     if (flag != "-E") {
//         std::cerr << "Expected first argument to be '-E'" << std::endl;
//         return 1;
//     }
//     std::string input_line;
//     std::getline(std::cin, input_line);
//     try {
//         if (match_pattern(input_line, pattern)) {
//             return 0;
//         } else {
//             return 1;
//         }
//     } catch (const std::runtime_error& e) {
//         std::cerr << e.what() << std::endl;
//         return 1;
//     }
// }