<script lang="ts">
  import { onMount } from 'svelte';
  import { profiles, activeProfile, activeProfileId, switchProfile, isSwitching, switchLatency } from '../stores/profiles';
  import type { Profile } from '../types';

  let dialElement: HTMLElement;
  let isDragging = false;
  let currentRotation = -120; // default to first profile

  // Define angular positions for up to 6 profiles across 240 deg arc (-120 to +120)
  const arcSpan = 240;
  const startAngle = -120;

  function getAngleForIndex(index: number, total: number): number {
    if (total <= 1) return 0;
    const step = arcSpan / (total - 1);
    return startAngle + index * step;
  }

  // Update rotation when active profile changes
  $: if ($profiles.length > 0 && $activeProfile) {
    const idx = $profiles.findIndex(p => p.id === $activeProfile.id);
    if (idx >= 0) {
      currentRotation = getAngleForIndex(idx, $profiles.length);
    }
  }

  function handleTickClick(p: Profile) {
    if (p.id !== $activeProfileId) {
      switchProfile(p.id);
    }
  }

  function handleMouseDown(e: MouseEvent) {
    isDragging = true;
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', handleMouseUp);
  }

  function handleMouseMove(e: MouseEvent) {
    if (!isDragging || !dialElement) return;
    const rect = dialElement.getBoundingClientRect();
    const centerX = rect.left + rect.width / 2;
    const centerY = rect.top + rect.height / 2;

    const deltaX = e.clientX - centerX;
    const deltaY = e.clientY - centerY;

    // Angle in degrees from 12 o'clock
    let angle = Math.atan2(deltaY, deltaX) * (180 / Math.PI) + 90;
    if (angle > 180) angle -= 360;

    // Clamp to -130 .. +130
    if (angle < -125) angle = -120;
    if (angle > 125) angle = 120;

    currentRotation = angle;

    // Find closest profile
    let closestIndex = 0;
    let minDiff = Infinity;
    $profiles.forEach((_, i) => {
      const targetAngle = getAngleForIndex(i, $profiles.length);
      const diff = Math.abs(angle - targetAngle);
      if (diff < minDiff) {
        minDiff = diff;
        closestIndex = i;
      }
    });

    const targetProfile = $profiles[closestIndex];
    if (targetProfile && targetProfile.id !== $activeProfileId) {
      switchProfile(targetProfile.id);
    }
  }

  function handleMouseUp() {
    isDragging = false;
    window.removeEventListener('mousemove', handleMouseMove);
    window.removeEventListener('mouseup', handleMouseUp);

    // Snap to closest profile
    const idx = $profiles.findIndex(p => p.id === $activeProfileId);
    if (idx >= 0) {
      currentRotation = getAngleForIndex(idx, $profiles.length);
    }
  }

  function handleDialKeydown(e: KeyboardEvent) {
    if ($profiles.length === 0) return;
    const currentIdx = Math.max(0, $profiles.findIndex(p => p.id === $activeProfileId));
    if (e.key === 'ArrowRight' || e.key === 'ArrowDown') {
      e.preventDefault();
      const next = $profiles[(currentIdx + 1) % $profiles.length];
      if (next) switchProfile(next.id);
    } else if (e.key === 'ArrowLeft' || e.key === 'ArrowUp') {
      e.preventDefault();
      const prev = $profiles[(currentIdx - 1 + $profiles.length) % $profiles.length];
      if (prev) switchProfile(prev.id);
    }
  }

  // Pre-calculated tick positions around the dial (percentage based)
  function getTickCoordinates(index: number, total: number) {
    const angleDeg = getAngleForIndex(index, total);
    // Convert angle where 0 is up, negative is left, positive is right
    // Standard polar coordinates (0 deg is top)
    const rad = (angleDeg - 90) * (Math.PI / 180);
    const radius = 43; // % radius from center
    const x = 50 + radius * Math.cos(rad);
    const y = 50 + radius * Math.sin(rad);
    return { x, y, angle: angleDeg };
  }
</script>

<div class="relative my-4 flex flex-col items-center justify-center z-10 w-full max-w-[580px] select-none">
  <!-- Top Mini Telemetry Header -->
  <div class="w-full flex items-center justify-between text-xs mb-3">
    <div class="flex items-center gap-3">
      <div class="flex items-center gap-1.5 px-2.5 py-1 rounded bg-[#0d141e] border border-[#1b2636] font-mono text-[11px] text-slate-300">
        <span class="text-slate-500">HOOK:</span>
        <span class="text-[#3fb950] font-medium">includeIf.gitdir</span>
        <span class="text-slate-600">|</span>
        <span class="text-indigo-400">ACTIVE</span>
      </div>
      <div class="hidden sm:flex items-center gap-1.5 px-2.5 py-1 rounded bg-[#0d141e] border border-[#1b2636] font-mono text-[11px] text-slate-400">
        <span class="text-slate-500">DAEMON:</span>
        <span class="text-emerald-400 font-mono font-medium">STRICT_DISPATCH ({$switchLatency}ms)</span>
      </div>
    </div>

    <!-- Mode Indicator / Dial Interaction Tip -->
    <div class="flex items-center gap-2 font-mono text-[11px] text-slate-400">
      <span class="inline-block w-2 h-2 rounded-full bg-[#3fb950] animate-pulse"></span>
      <span class="hidden md:inline">ROTATE OR CLICK PROFILE TICK TO SWITCH</span>
      <span class="md:hidden">CLICK TICK TO SWITCH</span>
    </div>
  </div>

  <!-- Circumference Glow Ring & Radial Gauge -->
  <div class="relative w-[340px] h-[340px] sm:w-[390px] sm:h-[390px] flex items-center justify-center">
    
    <!-- Outer Decorative Calibrated Gauge Ticks (VST Console style) -->
    <svg class="absolute inset-0 w-full h-full pointer-events-none" viewBox="0 0 400 400">
      <defs>
        <radialGradient id="ringGlow" cx="50%" cy="50%" r="50%">
          <stop offset="70%" stop-color="#3fb950" stop-opacity="0.15"></stop>
          <stop offset="100%" stop-color="#3fb950" stop-opacity="0"></stop>
        </radialGradient>
      </defs>

      <!-- Outer track circle -->
      <circle cx="200" cy="200" r="176" fill="none" stroke="#162230" stroke-width="2.5" stroke-dasharray="4 6"></circle>
      
      <!-- Arc track for rotation boundary (240 deg range) -->
      <circle cx="200" cy="200" r="152" fill="none" stroke="#0e1622" stroke-width="12"></circle>
      <path d="M 94 284 A 150 150 0 1 1 306 284" fill="none" stroke="#1c2a3a" stroke-width="4" stroke-linecap="round"></path>
      
      <!-- Precision Sub-tick marks generated along arc -->
      <g opacity="0.6">
        <line x1="200" y1="36" x2="200" y2="46" stroke="#2a3d52" stroke-width="2" transform="rotate(-120 200 200)"></line>
        <line x1="200" y1="38" x2="200" y2="44" stroke="#1c2c3d" stroke-width="1.5" transform="rotate(-100 200 200)"></line>
        <line x1="200" y1="38" x2="200" y2="44" stroke="#1c2c3d" stroke-width="1.5" transform="rotate(-80 200 200)"></line>
        <line x1="200" y1="38" x2="200" y2="44" stroke="#1c2c3d" stroke-width="1.5" transform="rotate(-60 200 200)"></line>
        <line x1="200" y1="36" x2="200" y2="46" stroke="#2a3d52" stroke-width="2" transform="rotate(-40 200 200)"></line>
        <line x1="200" y1="38" x2="200" y2="44" stroke="#1c2c3d" stroke-width="1.5" transform="rotate(-20 200 200)"></line>
        <line x1="200" y1="36" x2="200" y2="47" stroke="#3fb950" stroke-width="2.5" transform="rotate(0 200 200)"></line>
        <line x1="200" y1="38" x2="200" y2="44" stroke="#1c2c3d" stroke-width="1.5" transform="rotate(20 200 200)"></line>
        <line x1="200" y1="36" x2="200" y2="46" stroke="#2a3d52" stroke-width="2" transform="rotate(40 200 200)"></line>
        <line x1="200" y1="38" x2="200" y2="44" stroke="#1c2c3d" stroke-width="1.5" transform="rotate(60 200 200)"></line>
        <line x1="200" y1="38" x2="200" y2="44" stroke="#1c2c3d" stroke-width="1.5" transform="rotate(80 200 200)"></line>
        <line x1="200" y1="38" x2="200" y2="44" stroke="#1c2c3d" stroke-width="1.5" transform="rotate(100 200 200)"></line>
        <line x1="200" y1="36" x2="200" y2="46" stroke="#2a3d52" stroke-width="2" transform="rotate(120 200 200)"></line>
      </g>
    </svg>

    <!-- DYNAMIC PROFILE CIRCUMFERENCE NODES / TICKS -->
    {#each $profiles as p, i}
      {@const coords = getTickCoordinates(i, $profiles.length)}
      {@const isActive = p.id === $activeProfileId}
      <button
        type="button"
        class="tick-point absolute cursor-pointer group z-20 transition-all duration-300 bg-transparent border-0 p-0 focus:outline-none"
        style="left: {coords.x}%; top: {coords.y}%; transform: translate(-50%, -50%);"
        on:click={() => handleTickClick(p)}
      >
        <div class="flex flex-col items-center">
          <div 
            class="tick-badge px-2.5 py-1 rounded-full font-mono text-[11px] font-medium flex items-center gap-1.5 transition-all shadow-md whitespace-nowrap
              {isActive 
                ? 'bg-[#101824] border text-white font-bold ring-2 ring-primary/20' 
                : 'bg-[#0d141e] border border-[#223348] text-slate-400 hover:text-slate-200 hover:border-slate-500'}"
            style="{isActive ? `border-color: ${p.color || '#3fb950'}; color: ${p.color || '#3fb950'}; box-shadow: 0 0 16px ${p.color}40;` : ''}"
          >
            <span 
              class="w-2 h-2 rounded-full {isActive ? 'animate-pulse' : 'bg-slate-600'}"
              style="{isActive ? `background-color: ${p.color || '#3fb950'};` : ''}"
            ></span>
            <span>{String(i + 1).padStart(2, '0')}. {p.name.toUpperCase()}</span>
          </div>
          <span class="text-[10px] font-mono mt-1 opacity-80 {isActive ? 'text-emerald-400 font-semibold' : 'text-slate-500'}">
            {p.tag || p.username}
          </span>
        </div>
      </button>
    {/each}

    <!-- Rotary Knob Housing (Outer Chassis) -->
    <div class="relative w-[210px] h-[210px] sm:w-[236px] sm:h-[236px] rounded-full bg-[#0b1118] border-4 border-[#162230] dial-outer-track p-3 flex items-center justify-center">
      
      <!-- Metallic Knurled Grip Ring (Outer bezel) -->
      <div class="w-full h-full rounded-full knurled-edge p-2.5 shadow-2xl flex items-center justify-center">
        
        <!-- Rotary Dial Cap (Rotates dynamically) -->
        <div 
          bind:this={dialElement}
          id="rotaryDial" 
          role="slider"
          tabindex="0"
          aria-label="Profile rotary selector"
          aria-valuemin={0}
          aria-valuemax={Math.max(0, $profiles.length - 1)}
          aria-valuenow={Math.max(0, $profiles.findIndex(p => p.id === $activeProfileId))}
          class="dial-cap w-full h-full rounded-full relative flex items-center justify-center cursor-grab active:cursor-grabbing focus:outline-none focus:ring-2 focus:ring-[#3fb950]/40"
          style="transform: rotate({currentRotation}deg); {isDragging ? 'transition: none;' : ''}"
          on:mousedown={handleMouseDown}
          on:keydown={handleDialKeydown}
        >
          
          <!-- Dial Needle / Position Indicator Notch (VST style) -->
          <div 
            class="absolute top-2 w-1.5 h-6 rounded-full dial-needle transition-all"
            style="background-color: {$activeProfile?.color || '#3fb950'}; box-shadow: 0 0 10px {$activeProfile?.color || '#3fb950'}, 0 0 18px {$activeProfile?.color || '#3fb950'};"
          ></div>
          
          <!-- Inner Bezel Ring with subtle metal brush look -->
          <div class="w-[124px] h-[124px] sm:w-[140px] sm:h-[140px] rounded-full bg-[#0a0f16] border border-[#213042] flex items-center justify-center relative shadow-inner">
            
            <!-- Glowing Accent Aura behind GitHub Icon -->
            <div 
              class="absolute inset-3 rounded-full blur-md transition-all duration-300"
              style="background-color: {$activeProfile?.color || '#3fb950'}26;"
            ></div>

            <!-- Central GitHub Icon Emblem & Active Label -->
            <div 
              class="relative z-10 flex flex-col items-center justify-center pointer-events-none"
              style="transform: rotate({-currentRotation}deg);"
            >
              <svg class="w-10 h-10 sm:w-11 sm:h-11 text-white drop-shadow-[0_2px_12px_rgba(255,255,255,0.25)] transition-transform hover:scale-105" viewBox="0 0 24 24" fill="currentColor">
                <path fill-rule="evenodd" clip-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.53 1.032 1.53 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"></path>
              </svg>
              <span 
                class="text-[9px] font-mono tracking-widest font-bold mt-1 uppercase"
                style="color: {$activeProfile?.color || '#3fb950'};"
              >
                {$activeProfile?.name || 'ACTIVE'}
              </span>
            </div>

          </div>

        </div>
      </div>
    </div>

  </div>
</div>
