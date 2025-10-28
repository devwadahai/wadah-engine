# Customer Support Agent

AI-powered customer support agent with empathy, knowledge base access, and escalation capabilities.

## Features

- 💬 **Natural Conversations**: Human-like, empathetic responses
- 📚 **Knowledge Base**: Search and retrieve company information
- 🎫 **Ticket Management**: Create and update support tickets
- 📊 **Full Tracing**: Audit all customer interactions
- ⚡ **Fast Responses**: Sub-second response times

## Quick Start

```bash
# 1. Set API key
export OPENAI_API_KEY='your-key'

# 2. Run the agent
wadah run wadah.yaml --interactive

# 3. Start helping customers
"Hi, I'm having trouble logging in"
"How do I reset my password?"
"I need to cancel my subscription"
```

## Example Interactions

### Password Reset
```
Customer: I can't log into my account
Agent: I'm sorry to hear you're having trouble logging in. Let me help you with that.
      
      Can you tell me:
      1. Are you getting an error message?
      2. Have you tried resetting your password?
      
      I can guide you through the password reset process right now.
```

### Feature Question
```
Customer: Does your product support API webhooks?
Agent: Great question! Yes, we do support webhooks. 
      
      Here's how to set them up:
      1. Go to Settings > Integrations
      2. Click "Add Webhook"
      3. Enter your endpoint URL
      
      Would you like me to send you our webhook documentation?
```

### Escalation
```
Customer: I need a refund NOW
Agent: I completely understand your frustration, and I want to help.
      
      For refund requests, I'll need to connect you with our billing team
      who can process this immediately. Let me create a priority ticket
      for you.
      
      Your ticket #12345 is created. Our team will reach out within 1 hour.
```

## Configuration

### Customization

Edit `prompts/system.txt` to:
- Add company-specific policies
- Include product knowledge
- Define escalation criteria
- Set response tone

### Knowledge Base

Add FAQs and documentation to `memory/`:
- `faqs.json` - Common questions
- `policies.txt` - Company policies
- `troubleshooting.md` - Technical guides

### Monitoring

Track key metrics:
- Response time
- Resolution rate
- Escalation frequency
- Customer satisfaction
- Budget usage

## Best Practices

### Do's
- ✅ Start with empathy
- ✅ Ask clarifying questions
- ✅ Provide step-by-step solutions
- ✅ Verify resolution
- ✅ Offer additional help

### Don'ts
- ❌ Make promises you can't keep
- ❌ Argue with customers
- ❌ Use technical jargon
- ❌ Ignore escalation signals
- ❌ Rush through issues

## Use Cases

### Tier 1 Support
- Password resets
- Account questions
- Feature explanations
- Basic troubleshooting

### Pre-Sales
- Product information
- Pricing questions
- Feature comparisons
- Demo requests

### Post-Sales
- Onboarding help
- Feature adoption
- Best practices
- Success tips

## Integration

### With Existing Systems

Connect to:
- **CRM**: Salesforce, HubSpot
- **Helpdesk**: Zendesk, Intercom
- **Chat**: Slack, Discord
- **Analytics**: Mixpanel, Amplitude

### Deployment Options

- **Live Chat**: Embed on website
- **Slack Bot**: Internal support
- **API**: Integrate with apps
- **Email**: Automated responses

## Metrics

Track performance:
- **Volume**: Queries per day
- **Speed**: Average response time
- **Quality**: Resolution rate
- **Cost**: Budget per interaction
- **Satisfaction**: Customer ratings

## Security & Privacy

- Customer data not stored
- Conversations fully traced
- Budget limits prevent abuse
- PII handling compliant
- Audit logs for compliance

## Requirements

- OpenAI API key
- (Optional) Knowledge base data
- (Optional) Ticket system integration

## License

Apache 2.0

