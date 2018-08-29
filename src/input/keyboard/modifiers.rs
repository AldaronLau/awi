// Copyright Jeron A. Lau 2017-2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use Event;

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

	pub fn update(&mut self, queue: &mut Vec<Event>, input: Event) -> () {
		match input {
			Event::Text(_) => match self.held {
				NONE | SHIFT => {},
				_ => return, // Ctrl,Shift,Alt shouldn't print.
			},
			Event::LCtrl(state) | Event::RCtrl(state) => {
				if state.is_some() {
					self.held |= CTRL
				} else {
					self.held &= !CTRL
				}
			},
			Event::LShift(state) | Event::RShift(state) => {
				if state.is_some() {
					self.held |= SHIFT
				} else {
					self.held &= !SHIFT
				}
			},
			Event::Alt(state) => {
				if state.is_some() {
					self.held |= ALT
				} else {
					self.held &= !ALT
				}
			},
			Event::Compose(state) => {
				// Toggle compose state.
				if state.is_some() {
					if self.held & COMPOSE == 0 {
						self.held |= COMPOSE
					} else {
						self.held &= !COMPOSE
					}
				}
			},
			Event::A(state) => if state.is_some() { self.a(queue) },
			Event::B(state) => if state.is_some() { self.b(queue) },
			Event::C(state) => if state.is_some() { self.c(queue) },
			Event::D(state) => if state.is_some() { self.d(queue) },
			Event::E(state) => if state.is_some() { self.e(queue) },
			Event::F(state) => if state.is_some() { self.f(queue) },
			Event::G(state) => if state.is_some() { self.g(queue) },
			Event::H(state) => if state.is_some() { self.h(queue) },
			Event::I(state) => if state.is_some() { self.i(queue) },
			Event::J(state) => if state.is_some() { self.j(queue) },
			Event::K(state) => if state.is_some() { self.k(queue) },
			Event::L(state) => if state.is_some() { self.l(queue) },
			Event::M(state) => if state.is_some() { self.m(queue) },
			Event::N(state) => if state.is_some() { self.n(queue) },
			Event::O(state) => if state.is_some() { self.o(queue) },
			Event::P(state) => if state.is_some() { self.p(queue) },
			Event::Q(state) => if state.is_some() { self.q(queue) },
			Event::R(state) => if state.is_some() { self.r(queue) },
			Event::S(state) => if state.is_some() { self.s(queue) },
			Event::T(state) => if state.is_some() { self.t(queue) },
			Event::U(state) => if state.is_some() { self.u(queue) },
			Event::V(state) => if state.is_some() { self.v(queue) },
			Event::W(state) => if state.is_some() { self.w(queue) },
			Event::X(state) => if state.is_some() { self.x(queue) },
			Event::Y(state) => if state.is_some() { self.y(queue) },
			Event::Z(state) => if state.is_some() { self.z(queue) },
			Event::Enter(state) => if state.is_some() { self.enter(queue) },
			Event::Apostrophe(state) => if state.is_some() { self.apostrophe(queue) },
			Event::Semicolon(state) => if state.is_some() { self.semicolon(queue) },
			Event::EqualSign(state) => if state.is_some() { self.equalsign(queue) },
			Event::Minus(state) => if state.is_some() { self.minus(queue) },
			Event::Num1(state) => if state.is_some() { self.num1(queue) },
			Event::Num2(state) => if state.is_some() { self.num2(queue) },
			Event::Num3(state) => if state.is_some() { self.num3(queue) },
			Event::Num4(state) => if state.is_some() { self.num4(queue) },
			Event::Num5(state) => if state.is_some() { self.num5(queue) },
			Event::Num6(state) => if state.is_some() { self.num6(queue) },
			Event::Num7(state) => if state.is_some() { self.num7(queue) },
			Event::Num8(state) => if state.is_some() { self.num8(queue) },
			Event::Num9(state) => if state.is_some() { self.num9(queue) },
			Event::Num0(state) => if state.is_some() { self.num0(queue) },
			_ => {},
		}
		queue.push(input)
	}

	fn a(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::Select,
			ALT => return, // TODO: Aldaron's OS: To App Screen
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn b(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn c(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::Copy,
			ALT => Event::Cancel,
			_ => return,
		})
	}

	fn d(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::Delete,
			ALT => Event::Text('δ'),
			ALT_SHIFT => Event::Text('Δ'),
			_ => return,
		})
	}

	fn e(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => Event::Text('ə'),
			ALT_SHIFT => Event::Text('€'),
			_ => return,
		})
	}

	fn f(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::Find,
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn g(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: Toggle Graphics / Terminal Mode
			_ => return,
		})
	}

	fn h(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::Help,
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn i(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::EmphasisItalic, // 𝘢
			CTRL_SHIFT => Event::Info, // 🛈
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn j(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn k(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn l(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::AlignLeft,
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn m(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn n(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::Exit, // TODO: New Session.
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn o(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: File Open Popup Window
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn p(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::Print,
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn q(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::Exit,
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn r(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn s(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::Share, // 🔗 Share TODO: Popup
			CTRL_SHIFT => Event::SaveCopy, //⭳ TODO: FileSys Popup
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn t(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::Open(None),
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn u(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::EmphasisUnderline,//⎁
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn v(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::Paste,
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn w(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::Close,
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn x(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::Cut,
			ALT => Event::Text('×'),
			_ => return,
		})
	}

	fn y(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::Redo,
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn z(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::Undo,
			CTRL_SHIFT => Event::Redo,
			ALT => Event::Text('÷'),
			_ => return,
		})
	}

	fn enter(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::AlignJustified,
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn apostrophe(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::AlignRight,
			ALT => return, // TODO: What does it do?
			_ => return,
		})
	}

	fn semicolon(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::AlignCenter,
			ALT => Event::Text('°'),
			_ => return,
		})
	}

	fn equalsign(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::EmphasisDoubleUnderline,
			_ => return,
		})
	}

	fn minus(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::EmphasisStrikeOut,
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn num1(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: Aldaron's OS / No OS: 🔈 MUTE 🔇
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn num2(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: Aldaron's OS / No OS: 🔈 Volume - 🔉
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn num3(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: Aldaron's OS / No OS: 🔈 Volume + 🔊
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn num4(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: Aldaron's OS / No OS: ⏯ Play⏵,Pause⏸
			_ => return,
		})
	}

	#[allow(unreachable_code)]
	fn num5(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => return, // TODO: What does it do?
			ALT => return, // TODO: Aldaron's OS / No OS: ⏹ Stop
			_ => return,
		})
	}

	fn num6(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::EmphasisBrokenUnderline,//⎂,
			ALT => return, // TODO: Aldaron's OS / No OS: ⏮ Track
			_ => return,
		})
	}

	fn num7(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::EmphasisOverline,
			ALT => return, // TODO: Aldaron's OS / No OS: ⏭ Track
			_ => return,
		})
	}

	fn num8(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::EmphasisBold,
			ALT => return, // TODO: Brightness ☀ - 🔅
			_ => return,
		})
	}

	fn num9(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::EmphasisInvertColor,
			ALT => return, // TODO: Brightness ☀ + 🔆
			_ => return,
		})
	}

	fn num0(&self, queue: &mut Vec<Event>) -> () {
		queue.push(match self.held & 0b0000_1111 {
			CTRL => Event::EmphasisNone,
			ALT => return, // TODO: Toggle Monitor Config 🖵
			_ => return,
		})
	}
}
