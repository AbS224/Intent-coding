// Enhanced Digital Twin with Cloudflare Worker Integration
// Add this script to replace the queryTwin function in digital-twin.html

async function queryTwin() {
    const input = document.querySelector('.twin-input').value;
    const response = document.getElementById('twinResponse');
    
    if (!input.trim()) return;
    
    response.innerHTML = '<div class="loading"></div> Querying AURA Cognitive Core via Cloudflare Workers...';
    
    try {
        // Replace 'your-worker-domain' with your actual Cloudflare Worker URL
        const workerResponse = await fetch('https://your-worker-domain.workers.dev/query', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ query: input })
        });
        
        const data = await workerResponse.json();
        
        if (data.response) {
            // Typewriter effect for response
            response.innerHTML = '';
            const text = data.response;
            let i = 0;
            
            function typeWriter() {
                if (i < text.length) {
                    response.innerHTML += text.charAt(i);
                    i++;
                    setTimeout(typeWriter, 30);
                } else {
                    // Add signature after completion
                    response.innerHTML += '\n\n— AURA Cognitive Core, Verifiable Proof Systems';
                }
            }
            
            typeWriter();
        } else {
            response.innerHTML = 'Error: Unable to process query through AURA Cognitive Core.';
        }
        
    } catch (error) {
        console.error('Digital Twin query failed:', error);
        response.innerHTML = `Connection to AURA Cognitive Core failed. Operating in offline mode...\n\nThe Crucible Engine represents my commitment to "Correct by Design, Not by Debugging" - a philosophy born from years of physical infrastructure auditing where failures had real-world consequences. Every formal proof we generate is a step toward deterministic assurance in an uncertain world.`;
    }
    
    // Clear input after query
    document.querySelector('.twin-input').value = '';
}

// Enhanced particle system with deterministic behavior
function createDeterministicParticle() {
    const particle = document.createElement('div');
    particle.className = 'particle';
    
    // Deterministic positioning based on timestamp
    const time = Date.now();
    const x = (time * 0.1) % window.innerWidth;
    const y = window.innerHeight;
    
    particle.style.left = x + 'px';
    particle.style.top = y + 'px';
    
    // Deterministic animation duration
    const duration = 3000 + (time % 2000);
    particle.style.animation = `float ${duration}ms linear`;
    
    document.getElementById('particles').appendChild(particle);
    
    setTimeout(() => {
        if (particle.parentNode) {
            particle.parentNode.removeChild(particle);
        }
    }, duration);
}

// Initialize deterministic particle system
setInterval(createDeterministicParticle, 800);

// Add Web Audio API for deterministic signal generation
class DeterministicAudioSignal {
    constructor() {
        this.audioContext = null;
        this.oscillator = null;
        this.gainNode = null;
        this.isPlaying = false;
    }
    
    async initialize() {
        if (!this.audioContext) {
            this.audioContext = new (window.AudioContext || window.webkitAudioContext)();
            
            // Create deterministic drone
            this.oscillator = this.audioContext.createOscillator();
            this.gainNode = this.audioContext.createGain();
            
            // Frequency based on mathematical constant (phi)
            const phi = (1 + Math.sqrt(5)) / 2;
            this.oscillator.frequency.setValueAtTime(55 * phi, this.audioContext.currentTime);
            
            // Low volume ambient drone
            this.gainNode.gain.setValueAtTime(0.05, this.audioContext.currentTime);
            
            this.oscillator.connect(this.gainNode);
            this.gainNode.connect(this.audioContext.destination);
            
            this.oscillator.start();
            this.isPlaying = true;
        }
    }
    
    toggle() {
        if (this.isPlaying) {
            this.gainNode.gain.setValueAtTime(0, this.audioContext.currentTime);
            this.isPlaying = false;
        } else {
            this.gainNode.gain.setValueAtTime(0.05, this.audioContext.currentTime);
            this.isPlaying = true;
        }
    }
}

// Initialize audio system
const audioSignal = new DeterministicAudioSignal();

// Add audio toggle button
document.addEventListener('DOMContentLoaded', () => {
    const audioButton = document.createElement('button');
    audioButton.innerHTML = '🔊 Signal';
    audioButton.className = 'twin-button';
    audioButton.style.position = 'fixed';
    audioButton.style.bottom = '20px';
    audioButton.style.left = '20px';
    audioButton.style.zIndex = '1000';
    
    audioButton.addEventListener('click', async () => {
        if (!audioSignal.audioContext) {
            await audioSignal.initialize();
            audioButton.innerHTML = '🔇 Signal';
        } else {
            audioSignal.toggle();
            audioButton.innerHTML = audioSignal.isPlaying ? '🔇 Signal' : '🔊 Signal';
        }
    });
    
    document.body.appendChild(audioButton);
});

// Add keyboard shortcuts
document.addEventListener('keydown', (e) => {
    // Ctrl/Cmd + Enter to query twin
    if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
        queryTwin();
    }
    
    // Escape to clear input
    if (e.key === 'Escape') {
        document.querySelector('.twin-input').value = '';
        document.querySelector('.twin-input').focus();
    }
});

// Auto-focus input on load
document.addEventListener('DOMContentLoaded', () => {
    document.querySelector('.twin-input').focus();
});