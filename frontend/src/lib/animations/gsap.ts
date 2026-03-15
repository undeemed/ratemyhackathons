import { gsap } from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';

let registered = false;

function ensureRegistered() {
	if (!registered && typeof window !== 'undefined') {
		gsap.registerPlugin(ScrollTrigger);
		registered = true;
	}
}

export function fadeIn(node: HTMLElement, params?: { delay?: number; duration?: number; y?: number }) {
	ensureRegistered();
	const { delay = 0, duration = 0.8, y = 40 } = params ?? {};
	const tween = gsap.fromTo(
		node,
		{ opacity: 0, y },
		{
			opacity: 1, y: 0, duration, delay,
			ease: 'power2.out',
			scrollTrigger: { trigger: node, start: 'top 85%', toggleActions: 'play none none none' },
		}
	);
	const st = tween.scrollTrigger;
	return { destroy() { st?.kill(); tween.kill(); } };
}

export function slideUp(node: HTMLElement, params?: { delay?: number; duration?: number }) {
	ensureRegistered();
	const { delay = 0, duration = 0.8 } = params ?? {};
	const tween = gsap.fromTo(
		node,
		{ opacity: 0, y: 60 },
		{
			opacity: 1, y: 0, duration, delay,
			ease: 'power3.out',
			scrollTrigger: { trigger: node, start: 'top 85%', toggleActions: 'play none none none' },
		}
	);
	const st = tween.scrollTrigger;
	return { destroy() { st?.kill(); tween.kill(); } };
}

export function staggerChildren(node: HTMLElement, params?: { stagger?: number; y?: number }) {
	ensureRegistered();
	const { stagger = 0.1, y = 30 } = params ?? {};
	const tween = gsap.fromTo(
		node.children,
		{ opacity: 0, y },
		{
			opacity: 1, y: 0, duration: 0.6, stagger,
			ease: 'power2.out',
			scrollTrigger: { trigger: node, start: 'top 85%', toggleActions: 'play none none none' },
		}
	);
	const st = tween.scrollTrigger;
	return { destroy() { st?.kill(); tween.kill(); } };
}

export function globeReveal(node: HTMLElement) {
	ensureRegistered();
	const tween = gsap.fromTo(
		node,
		{ opacity: 0, x: 200, scale: 0.7, rotation: -40 },
		{
			opacity: 1, x: 0, scale: 1, rotation: 0,
			ease: 'none',
			scrollTrigger: {
				trigger: node,
				start: 'top 95%',
				end: 'top 30%',
				scrub: 1,
			},
		}
	);
	const st = tween.scrollTrigger;
	return { destroy() { st?.kill(); tween.kill(); } };
}

export function countUp(node: HTMLElement, params: { target: number; duration?: number; prefix?: string; suffix?: string }) {
	ensureRegistered();
	const { target, duration = 2, prefix = '', suffix = '' } = params;
	const obj = { val: 0 };
	let lastDisplay = -1;
	const tween = gsap.to(obj, {
		val: target, duration,
		ease: 'power1.out',
		scrollTrigger: { trigger: node, start: 'top 85%', toggleActions: 'play none none none' },
		onUpdate() {
			const rounded = Math.round(obj.val);
			if (rounded !== lastDisplay) {
				lastDisplay = rounded;
				node.textContent = `${prefix}${rounded.toLocaleString()}${suffix}`;
			}
		},
	});
	const st = tween.scrollTrigger;
	return { destroy() { st?.kill(); tween.kill(); } };
}
