/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

// little hard
#[derive(Debug)]
struct Stack<T> {
	size: usize,
	data: Vec<T>,
}
impl<T> Stack<T> {
	fn new() -> Self {
		Self {
			size: 0,
			data: Vec::new(),
		}
	}
	fn is_empty(&self) -> bool {
		0 == self.size
	}
	fn len(&self) -> usize {
		self.size
	}
	fn clear(&mut self) {
		self.size = 0;
		self.data.clear();
	}
	fn push(&mut self, val: T) {
		self.data.push(val);
		self.size += 1;
	}
	fn pop(&mut self) -> Option<T> {
		// TODO
		// None
		let result = self.data.pop();
		self.size -= 1;
		result
	}
	fn peek(&self) -> Option<&T> {
		if 0 == self.size {
			return None;
		}
		self.data.get(self.size - 1)
	}
	fn peek_mut(&mut self) -> Option<&mut T> {
		if 0 == self.size {
			return None;
		}
		self.data.get_mut(self.size - 1)
	}
	fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
	fn iter(&self) -> Iter<T> {
		let mut iterator = Iter { 
			stack: Vec::new() 
		};
		for item in self.data.iter() {
			iterator.stack.push(item);
		}
		iterator
	}
	fn iter_mut(&mut self) -> IterMut<T> {
		let mut iterator = IterMut { 
			stack: Vec::new() 
		};
		for item in self.data.iter_mut() {
			iterator.stack.push(item);
		}
		iterator
	}
}
struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		if !self.0.is_empty() {
			self.0.size -= 1;self.0.data.pop()
		} 
		else {
			None
		}
	}
}
struct Iter<'a, T: 'a> {
	stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}
struct IterMut<'a, T: 'a> {
	stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}

fn bracket_match(bracket: &str) -> bool
{
	//TODO
	let mut stack = Stack::<char>::new(); // 存储压栈的左半括号
	let mut idx = 0;
	for c in bracket.chars(){ // 以char的形式读取braket
		if c == '[' || c == '{' || c == '(' { // 进行压栈
			stack.push(c);
		}else if c == ']' || c == '}' || c == ')' { // 进行出栈
			if stack.is_empty(){ // 栈空匹配失败
				return false;
			}
			let top_char = stack.peek().unwrap();
			if match_or_false(top_char,&c){ // 当前右括号与栈顶匹配，成功出栈，失败返回false
				stack.pop();
			}else {
				return false;
			}
		}
	}
	if stack.is_empty(){ // 字符匹配完后栈内仍有字符则匹配不充分
		true
	} else {
		false
	}
}

fn match_or_false(stack_top: &char, match_char: &char) -> bool{
	let mut result = false;
	if *stack_top == '(' && *match_char == ')'{
		result = true		
	}else if *stack_top == '[' && *match_char == ']'{
		result = true		
	}else if *stack_top == '{' && *match_char == '}'{
		result = true		
	}
	result
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bracket_matching_1(){
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_2(){
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_3(){
		let s = "{{([])}}";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_4(){
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_5(){
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_6(){
		let s = "";
		assert_eq!(bracket_match(s),true);
	}
}