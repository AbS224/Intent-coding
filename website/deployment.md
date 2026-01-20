# Digital Twin Portfolio Deployment Guide

## Cloudflare Workers + Pages Setup

### 1. Deploy the AI Worker

```bash
# Install Wrangler CLI
npm install -g wrangler

# Login to Cloudflare
wrangler login

# Create new worker
wrangler init digital-twin-api

# Copy worker.js content to src/index.js
# Deploy worker
wrangler deploy
```

### 2. Deploy Static Site to Cloudflare Pages

```bash
# Connect your GitHub repo to Cloudflare Pages
# Build settings:
# - Build command: (none)
# - Build output directory: website
# - Root directory: website
```

### 3. Domain Configuration

1. **Add Custom Domain**: verifiableproof.systems
2. **Configure DNS**: Point to Cloudflare Pages
3. **SSL/TLS**: Full (strict) encryption
4. **Security Headers**: Enable HSTS, CSP

### 4. Worker Integration

Update `digital-twin.js` with your worker URL:
```javascript
const workerResponse = await fetch('https://digital-twin-api.your-subdomain.workers.dev/query', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ query: input })
});
```

### 5. Performance Optimization

**Cloudflare Settings:**
- Enable Brotli compression
- Minify HTML, CSS, JS
- Enable Rocket Loader
- Set Browser Cache TTL: 4 hours
- Set Edge Cache TTL: 2 hours

**Worker Optimization:**
- Use KV storage for knowledge base (optional)
- Implement response caching
- Add rate limiting

### 6. Security Configuration

**Content Security Policy:**
```
default-src 'self'; 
script-src 'self' 'unsafe-inline'; 
style-src 'self' 'unsafe-inline'; 
connect-src 'self' https://*.workers.dev;
```

**Additional Headers:**
- X-Frame-Options: DENY
- X-Content-Type-Options: nosniff
- Referrer-Policy: strict-origin-when-cross-origin

### 7. Analytics & Monitoring

- Enable Cloudflare Web Analytics
- Set up Worker Analytics
- Configure Real User Monitoring (RUM)
- Add custom metrics for AI queries

### 8. Cost Optimization

**Free Tier Limits:**
- Workers: 100,000 requests/day
- Pages: Unlimited requests
- KV: 100,000 reads/day
- Bandwidth: Unlimited

**Estimated Monthly Cost:** $0 (within free limits)

### 9. File Structure

```
website/
├── index.html              # Main academic site
├── digital-twin.html       # Interactive portfolio
├── digital-twin.js         # Enhanced functionality
├── worker.js              # Cloudflare Worker code
└── deployment.md          # This file
```

### 10. Testing

**Local Development:**
```bash
# Test worker locally
wrangler dev

# Test static site
python -m http.server 8000
```

**Production Testing:**
- Test AI twin responses
- Verify audio signal generation
- Check particle system performance
- Validate mobile responsiveness

### 11. Maintenance

**Regular Updates:**
- Monitor worker performance
- Update knowledge base responses
- Refresh security headers
- Optimize particle animations

**Backup Strategy:**
- Git repository for code
- Cloudflare dashboard exports
- Worker script backups

This setup provides a completely free, high-performance digital twin portfolio with AI capabilities, hosted on Cloudflare's global edge network.