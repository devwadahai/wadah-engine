import os
from langchain_openai import ChatOpenAI, OpenAIEmbeddings
from langchain_core.prompts import ChatPromptTemplate
from langchain_core.runnables import RunnablePassthrough
from langchain_core.output_parsers import StrOutputParser
from langchain_community.vectorstores import Qdrant
from qdrant_client import QdrantClient

# Load system prompt
SYSTEM_PROMPT = open("prompts/system.txt").read()

# Initialize model (TGI via OpenAI-compatible API)
# OPENAI_BASE environment variable points to TGI
llm = ChatOpenAI(
    model=os.getenv("OPENAI_MODEL", "mistral-7b-instruct"),
    temperature=0.2
)

# Initialize embeddings
embeddings = OpenAIEmbeddings()

# Initialize Qdrant vector store
qdrant_client = QdrantClient(url=os.getenv("QDRANT_URL", "http://localhost:6333"))
vector_store = Qdrant(
    client=qdrant_client,
    collection_name="langchain_docs",
    embeddings=embeddings
)

# Create retriever
retriever = vector_store.as_retriever(
    search_kwargs={"k": 5}  # Top 5 results
)

# Create prompt template
prompt = ChatPromptTemplate.from_messages([
    ("system", SYSTEM_PROMPT),
    ("human", """Context from documentation:
{context}

Question: {question}

Answer based on the context above:""")
])

# Create RAG chain
rag_chain = (
    {"context": retriever, "question": RunnablePassthrough()}
    | prompt
    | llm
    | StrOutputParser()
)

def agent(question: str = ""):
    """
    Main agent entrypoint called by Wadah.
    
    Args:
        question: User question to answer
    
    Returns:
        dict with answer and metadata
    """
    if not question:
        return {"error": "No question provided"}
    
    # Run RAG chain
    answer = rag_chain.invoke(question)
    
    return {
        "question": question,
        "answer": answer,
        "model": "mistral-7b-instruct",
        "backend": "tgi + qdrant"
    }

if __name__ == "__main__":
    # For local testing
    test_question = "What is Wadah?"
    result = agent(test_question)
    print(f"Q: {result['question']}")
    print(f"A: {result['answer']}")

