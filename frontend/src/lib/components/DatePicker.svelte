<script lang="ts">
	let {
		value = $bindable(''),
		placeholder = 'Select date',
	}: {
		value: string;
		placeholder?: string;
	} = $props();

	let open = $state(false);
	let pickerEl: HTMLDivElement | undefined = $state();
	let triggerEl: HTMLButtonElement | undefined = $state();

	// Which month is being viewed
	let viewYear = $state(new Date().getFullYear());
	let viewMonth = $state(new Date().getMonth()); // 0-indexed

	// When value changes externally, sync the view
	$effect(() => {
		if (value) {
			const d = new Date(value + 'T00:00:00');
			viewYear = d.getFullYear();
			viewMonth = d.getMonth();
		}
	});

	const DAYS = ['Su', 'Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa'];
	const MONTHS = [
		'January', 'February', 'March', 'April', 'May', 'June',
		'July', 'August', 'September', 'October', 'November', 'December',
	];

	const calendarDays = $derived.by(() => {
		const first = new Date(viewYear, viewMonth, 1);
		const startDay = first.getDay(); // 0=Sun
		const daysInMonth = new Date(viewYear, viewMonth + 1, 0).getDate();

		const cells: (number | null)[] = [];
		for (let i = 0; i < startDay; i++) cells.push(null);
		for (let d = 1; d <= daysInMonth; d++) cells.push(d);
		return cells;
	});

	function displayValue(): string {
		if (!value) return '';
		const d = new Date(value + 'T00:00:00');
		return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' }).toUpperCase();
	}

	function prevMonth() {
		if (viewMonth === 0) {
			viewMonth = 11;
			viewYear--;
		} else {
			viewMonth--;
		}
	}

	function nextMonth() {
		if (viewMonth === 11) {
			viewMonth = 0;
			viewYear++;
		} else {
			viewMonth++;
		}
	}

	function selectDay(day: number) {
		const m = String(viewMonth + 1).padStart(2, '0');
		const d = String(day).padStart(2, '0');
		value = `${viewYear}-${m}-${d}`;
		open = false;
	}

	function isSelected(day: number): boolean {
		if (!value) return false;
		const m = String(viewMonth + 1).padStart(2, '0');
		const d = String(day).padStart(2, '0');
		return value === `${viewYear}-${m}-${d}`;
	}

	function isToday(day: number): boolean {
		const now = new Date();
		return day === now.getDate() && viewMonth === now.getMonth() && viewYear === now.getFullYear();
	}

	function handleClickOutside(e: MouseEvent) {
		if (!open) return;
		const target = e.target as Node;
		if (pickerEl?.contains(target) || triggerEl?.contains(target)) return;
		open = false;
	}

	function toggle() {
		open = !open;
		if (open && !value) {
			const now = new Date();
			viewYear = now.getFullYear();
			viewMonth = now.getMonth();
		}
	}
</script>

<svelte:window onclick={handleClickOutside} />

<div class="relative">
	<button
		bind:this={triggerEl}
		onclick={toggle}
		class="flex items-center gap-2 border border-border bg-bg px-2 py-1 text-xs text-text transition-colors hover:border-accent focus:border-accent focus:outline-none"
	>
		{#if value}
			<span class="tracking-wide">{displayValue()}</span>
		{:else}
			<span class="tracking-wide text-dim">{placeholder}</span>
		{/if}
		<span class="text-[8px] text-dim">{open ? '▲' : '▼'}</span>
	</button>

	{#if open}
		<div
			bind:this={pickerEl}
			class="absolute left-0 top-full z-50 mt-1 border border-border bg-bg p-3 shadow-[0_4px_24px_rgba(0,0,0,0.8)]"
			style="min-width: 260px;"
		>
			<!-- Month/Year nav -->
			<div class="mb-3 flex items-center justify-between">
				<button
					onclick={prevMonth}
					class="px-2 py-0.5 text-xs text-dim transition-colors hover:text-text"
				>←</button>
				<span class="text-[10px] uppercase tracking-[0.2em] text-text">
					{MONTHS[viewMonth]} {viewYear}
				</span>
				<button
					onclick={nextMonth}
					class="px-2 py-0.5 text-xs text-dim transition-colors hover:text-text"
				>→</button>
			</div>

			<!-- Day headers -->
			<div class="mb-1 grid grid-cols-7 gap-0">
				{#each DAYS as day}
					<span class="py-1 text-center text-[9px] uppercase tracking-[0.15em] text-dim">{day}</span>
				{/each}
			</div>

			<!-- Day grid -->
			<div class="grid grid-cols-7 gap-0">
				{#each calendarDays as cell}
					{#if cell === null}
						<span></span>
					{:else}
						<button
							onclick={() => selectDay(cell)}
							class="py-1.5 text-center text-xs transition-colors
								{isSelected(cell)
									? 'bg-text text-bg font-bold'
									: isToday(cell)
										? 'text-accent font-bold hover:bg-elevated'
										: 'text-text hover:bg-elevated'}"
						>
							{cell}
						</button>
					{/if}
				{/each}
			</div>

			<!-- Clear -->
			{#if value}
				<button
					onclick={() => { value = ''; open = false; }}
					class="mt-2 w-full border-t border-border pt-2 text-[10px] uppercase tracking-[0.2em] text-dim transition-colors hover:text-text"
				>
					Clear
				</button>
			{/if}
		</div>
	{/if}
</div>
