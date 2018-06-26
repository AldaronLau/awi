// "awi" - Aldaron's Window Interface
//
// Copyright Jeron A. Lau 2017-2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use afi_docf::{ Emphasis, Align };

use Input;

const NONE : u8 = 0b0000_0000;
const SHIFT : u8 = 0b0000_0001;
const CTRL : u8 = 0b0000_0010;
const ALT : u8 = 0b0000_0100;
const COMPOSE : u8 = 0b0000_1000;
const ALT_SHIFT : u8 = ALT | SHIFT;
const CTRL_SHIFT : u8 = CTRL | SHIFT;

pub(crate) struct Modifiers {
	held: u8,
}

impl Modifiers {
	pub fn create() -> Modifiers {
		Modifiers { held: NONE }
	}

	pub fn update(&mut self, queue: &mut Vec<Input>, input: Input) -> () {
		match input {
			Input::Text(_) => match self.held {
				NONE | SHIFT => {},
				_ => return, // Ctrl,Shift,Alt shouldn't print.
			},
			Input::LCtrl(state) | Input::RCtrl(state) => {
				if state.is_some() {
					self.held |= CTRL
				} else {
					self.held &= !CTRL
				}
			},
			Input::LShift(state) | Input::RShift(state) => {
				if state.is_some() {
					self.held |= SHIFT
				} else {
					self.held &= !SHIFT
				}
			},
			Input::Alt(state) => {
				if state.is_some() {
					self.held |= ALT
				} else {
					self.held &= !ALT
				}
			},
			Input::Compose(state) => {
				// Toggle compose state.
				if state.is_some() {
					if self.held & COMPOSE == 0 {
						self.held |= COMPOSE
					} else {
						self.held &= !COMPOSE
					}
				}
			},
			Input::A(state) => if state.is_some() { self.a(queue) },
			Input::B(state) => if state.is_some() { self.b(queue) },
			Input::C(state) => if state.is_some() { self.c(queue) },
			Input::D(state) => if state.is_some() { self.d(queue) },
			Input::E(state) => if state.is_some() { self.e(queue) },
			Input::F(state) => if state.is_some() { self.f(queue) },
			Input::G(state) => if state.is_some() { self.g(queue) },
			Input::H(state) => if state.is_some() { self.h(queue) },
			Input::I(state) => if state.is_some() { self.i(queue) },
			Input::J(state) => if state.is_some() { self.j(queue) },
			Input::K(state) => if state.is_some() { self.k(queue) },
			Input::L(state) => if state.is_some() { self.l(queue) },
			Input::M(state) => if state.is_some() { self.m(queue) },
			Input::N(state) => if state.is_some() { self.n(queue) },
			Input::O(state) => if state.is_some() { self.o(queue) },
			Input::P(state) => if state.is_some() { self.p(queue) },
			Input::Q(state) => if state.is_some() { self.q(queue) },
			Input::R(state) => if state.is_some() { self.r(queue) },
			Input::S(state) => if state.is_some() { self.s(queue) },
			Input::T(state) => if state.is_some() { self.t(queue) },
			Input::U(state) => if state.is_some() { self.u(queue) },
			Input::V(state) => if state.is_some() { self.v(queue) },
			Input::W(state) => if state.is_some() { self.w(queue) },
			Input::X(state) => if state.is_some() { self.x(queue) },
			Input::Y(state) => if state.is_some() { self.y(queue) },
			Input::Z(state) => if state.is_some() { self.z(queue) },
			Input::Enter(state) => if state.is_some() { self.enter(queue) },
			Input::Apostrophe(state) => if state.is_some() { self.apostrophe(queue) },
			Input::Semicolon(state) => if state.is_some() { self.semicolon(queue) },
			Input::EqualSign(state) => if state.is_some() { self.equalsign(queue) },
			Input::Minus(state) => if state.is_some() { self.minus(queue) },
			Input::Num1(state) => if state.is_some() { self.num1(queue) },
			Input::Num2(state) => if state.is_some() { self.num2(queue) },
			Input::Num3(state) => if state.is_some() { self.num3(queue) },
			Input::Num4(state) => if state.is_some() { self.num4(queue) },
			Input::Num5(state) => if state.is_some() { self.num5(queue) },
			Input::Num6(state) => if state.is_some() { self.num6(queue) },
			Input::Num7(state) => if state.is_some() { self.num7(queue) },
			Input::Num8(state) => if state.is_some() { self.num8(queue) },
			Input::Num9(state) => if state.is_some() { self.num9(queue) },
			Input::Num0(state) => if state.is_some() { self.num0(queue) },
			_ => {},
		}
		queue.push(input)
	}

	fn a(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Select,
			ALT => return, // TODO: Aldaron's OS: To App Screen
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn b(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn c(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Copy,
			ALT => Input::Cancel,
			_ => return,
		})
	}

	fn d(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Delete,
			ALT => Input::Text('δ'),
			ALT_SHIFT => Input::Text('Δ'),
			_ => return,
		})
	}

	fn e(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => Input::Text('ə'),
			ALT_SHIFT => Input::Text('€'),
			_ => return,
		})
	}

	fn f(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Find,
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn g(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: Toggle Graphics / Terminal Mode
			_ => return,
		})
	}

	fn h(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Help,
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn i(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Emphasis(Emphasis::Italic), // 𝘢
			CTRL_SHIFT => Input::Info, // 🛈
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn j(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn k(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn l(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Align(Align::Left),
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn m(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn n(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Quit, // TODO: New Session.
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn o(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: File Open Popup Window
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn p(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Print,
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn q(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Quit,
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn r(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn s(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Share, // 🔗 Share TODO: Popup
			CTRL_SHIFT => Input::SaveCopy, //⭳ TODO: FileSys Popup
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn t(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Open(None),
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn u(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Emphasis(Emphasis::Underline),//⎁
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn v(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Paste,
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn w(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Close,
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn x(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Cut,
			ALT => Input::Text('×'),
			_ => return,
		})
	}

	fn y(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Redo,
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn z(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Undo,
			CTRL_SHIFT => Input::Redo,
			ALT => Input::Text('÷'),
			_ => return,
		})
	}

	fn enter(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Align(Align::Justified),
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn apostrophe(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Align(Align::Right),
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn semicolon(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Align(Align::Centered),
			ALT => Input::Text('°'),
			_ => return,
		})
	}

	fn equalsign(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Emphasis(Emphasis::UnderlineX2),
			_ => return,
		})
	}

	fn minus(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Emphasis(Emphasis::StrikeOut),
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn num1(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: Aldaron's OS / No OS: 🔈 MUTE 🔇
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn num2(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: Aldaron's OS / No OS: 🔈 Volume - 🔉
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn num3(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: Aldaron's OS / No OS: 🔈 Volume + 🔊
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn num4(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: Aldaron's OS / No OS: ⏯ Play⏵,Pause⏸
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn num5(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: Aldaron's OS / No OS: ⏹ Stop
			_ => return,
		})
	}

	fn num6(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Emphasis(Emphasis::UnderlineDC),//⎂,
			ALT => return, // TODO: Aldaron's OS / No OS: ⏮ Track
			_ => return,
		})
	}

	fn num7(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Emphasis(Emphasis::Overline),
			ALT => return, // TODO: Aldaron's OS / No OS: ⏭ Track
			_ => return,
		})
	}

	fn num8(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Emphasis(Emphasis::Bold),
			ALT => return, // TODO: Brightness ☀ - 🔅
			_ => return,
		})
	}

	fn num9(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Emphasis(Emphasis::InvertColor),
			ALT => return, // TODO: Brightness ☀ + 🔆
			_ => return,
		})
	}

	fn num0(&self, queue: &mut Vec<Input>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Input::Emphasis(Emphasis::None),
			ALT => return, // TODO: Toggle Monitor Config 🖵
			_ => return,
		})
	}
}
