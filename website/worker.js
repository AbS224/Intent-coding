// Cloudflare Worker for WebLLM Digital Twin
// Deploy to Cloudflare Workers for free AI inference

export default {
  async fetch(request, env, ctx) {
    // CORS headers for browser requests
    const corsHeaders = {
      'Access-Control-Allow-Origin': '*',
      'Access-Control-Allow-Methods': 'GET, POST, OPTIONS',
      'Access-Control-Allow-Headers': 'Content-Type',
    };

    if (request.method === 'OPTIONS') {
      return new Response(null, { headers: corsHeaders });
    }

    if (request.method === 'POST' && new URL(request.url).pathname === '/query') {
      try {
        const { query } = await request.json();
        
        // Adam B. Straughn's knowledge base for the digital twin
        const knowledgeBase = {
          philosophy: `The transition from physical infrastructure auditing to AI safety engineering represents a fundamental shift from reactive to proactive verification. In my BP/Shell days, we audited after systems were built. Now, with Crucible Engine, we prove correctness before deployment.`,
          
          bicameral: `The "Bicameral AI Double Key" architecture ensures no single AI agent can make unilateral decisions. Like nuclear launch protocols, critical AI operations require consensus between multiple verification systems - the Blue Team (logic verification) and Red Team (adversarial testing).`,
          
          vibecoding: `"Vibecoding" emerged from recognizing that most requirements start as informal "vibes" - intuitive understanding of what a system should do. The Crucible Engine transforms these vibes into mathematically rigorous specifications through Tree-Sitter parsing and Z3 SMT solving.`,
          
          quantum: `Post-quantum cryptography isn't just about future-proofing - it's about creating audit trails that remain valid even as computing paradigms shift. ML-KEM and ML-DSA signatures ensure our verification certificates maintain integrity across technological transitions.`,
          
          hal9000: `The "HAL 9000 Fallacy" describes how science fiction narratives create unrealistic expectations about AI behavior. By providing verifiable proofs instead of black-box outputs, we shift evaluation from narrative belief to mathematical certainty.`,
          
          oregon: `Oregon represents the perfect intersection of technological innovation and environmental consciousness. The state's commitment to renewable energy aligns with my vision of sustainable, verifiable AI systems that don't waste computational resources on unnecessary iterations.`,
          
          redemption: `My journey from losing everything during COVID to building Crucible Engine embodies "Redemption as Engineering" - the idea that technical excellence can emerge from personal adversity. Every formal proof I write is a step away from chaos toward deterministic assurance.`,
          
          thunderdome: `The Thunderdome architecture pits AI agents against each other in adversarial verification. Blue Team agents propose solutions, Red Team agents attack them, and the Judge validates through formal methods. Only solutions that survive this gauntlet become production code.`,
          
          annie: `ANNIE (Agentic Neural Network for Intent Engineering) represents the synthesis of human intuition and machine precision. It's not about replacing human judgment, but about amplifying it through mathematical verification and adversarial testing.`,
          
          safety: `AI safety isn't about preventing robot uprisings - it's about ensuring systems behave predictably under all conditions. The Safety Kernel uses AND-gate logic: every operation must pass multiple independent verification checks before execution.`
        };
        
        // Simple keyword matching for responses
        const lowerQuery = query.toLowerCase();
        let response = "I appreciate your question about verifiable proof systems. ";
        
        if (lowerQuery.includes('philosophy') || lowerQuery.includes('approach') || lowerQuery.includes('transition')) {
          response += knowledgeBase.philosophy;
        } else if (lowerQuery.includes('bicameral') || lowerQuery.includes('double key') || lowerQuery.includes('architecture')) {
          response += knowledgeBase.bicameral;
        } else if (lowerQuery.includes('vibecoding') || lowerQuery.includes('vibe') || lowerQuery.includes('requirements')) {
          response += knowledgeBase.vibecoding;
        } else if (lowerQuery.includes('quantum') || lowerQuery.includes('cryptography') || lowerQuery.includes('security')) {
          response += knowledgeBase.quantum;
        } else if (lowerQuery.includes('hal') || lowerQuery.includes('science fiction') || lowerQuery.includes('narrative')) {
          response += knowledgeBase.hal9000;
        } else if (lowerQuery.includes('oregon') || lowerQuery.includes('location') || lowerQuery.includes('pacific')) {
          response += knowledgeBase.oregon;
        } else if (lowerQuery.includes('redemption') || lowerQuery.includes('journey') || lowerQuery.includes('covid') || lowerQuery.includes('personal')) {
          response += knowledgeBase.redemption;
        } else if (lowerQuery.includes('thunderdome') || lowerQuery.includes('adversarial') || lowerQuery.includes('verification')) {
          response += knowledgeBase.thunderdome;
        } else if (lowerQuery.includes('annie') || lowerQuery.includes('intent') || lowerQuery.includes('engineering')) {
          response += knowledgeBase.annie;
        } else if (lowerQuery.includes('safety') || lowerQuery.includes('kernel') || lowerQuery.includes('assurance')) {
          response += knowledgeBase.safety;
        } else {
          // Default response combining key concepts
          response += `My work focuses on transforming AI from unpredictable "black boxes" into verifiable, mathematically proven systems. Through the Crucible Engine's four-pillar architecture - HITL 2.0, Safety Kernel, ANNIE, and Thunderdome - we achieve "Correct by Design, Not by Debugging." This represents a fundamental shift from reactive debugging to proactive verification, ensuring AI systems are trustworthy before deployment.`;
        }
        
        return new Response(JSON.stringify({ response }), {
          headers: { ...corsHeaders, 'Content-Type': 'application/json' }
        });
        
      } catch (error) {
        return new Response(JSON.stringify({ 
          error: 'Processing failed',
          response: 'The AURA Cognitive Core is currently recalibrating. Please try your query again.'
        }), {
          status: 500,
          headers: { ...corsHeaders, 'Content-Type': 'application/json' }
        });
      }
    }
    
    // Health check endpoint
    if (request.method === 'GET' && new URL(request.url).pathname === '/health') {
      return new Response(JSON.stringify({ 
        status: 'operational',
        system: 'AURA Cognitive Core',
        version: '1.0.0',
        capabilities: ['verifiable-proof-systems', 'formal-verification', 'ai-safety']
      }), {
        headers: { ...corsHeaders, 'Content-Type': 'application/json' }
      });
    }
    
    return new Response('Digital Twin API - Verifiable Proof Systems', {
      headers: corsHeaders
    });
  }
};