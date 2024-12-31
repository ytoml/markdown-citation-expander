# Markdown Citation Expander

An Raycast script that expands citations in a markdown text copied to a clipboard.
Maybe useful when you want to copy outputs from tools like Perplexity or Felo to Notion or other markdown editors.

Only for MacOS.


### Input/Output example

Input (Recent tools like Perplexity or Felo has output like this):
```markdown
OpenAIは最近、新しい言語モデルの開発で注目を集めています。

2024年9月12日に「OpenAI o1」と「OpenAI o1-mini」という新しいモデルをリリースしました[1][3]。OpenAI o1は、特に理数系科目やコーディングにおいて高い性能を示し、複雑な問題解決能力が向上しています[1][3]。

OpenAI o1の主な特徴:
- 独自の「推論トークン」を使用し、複雑な問題に対応できる思考力を持つ[1]
- 理数科目やコーディングで専門家レベルの性能を発揮[1]
- プロンプトインジェクションへの耐性が向上[1]

OpenAI o1-miniは、理数系科目に特化した小型モデルで、処理速度が約3~5倍高速になり、コストも80%削減されています[1]。

さらに、2024年12月21日には「OpenAI o3」の初期バージョンが発表され、o1をさらに上回る性能を達成したと報告されています[2]。

これらの新モデルの登場により、OpenAIは生成AI市場でのリーダーシップを強化し、ユーザーの期待に応える革新的な技術を提供し続けているという評価を得ています[3]。

Citations:
[1] https://weel.co.jp/media/innovator/openai-o1/
[2] https://gihyo.jp/article/2024/12/openai-o3-early-version
[3] https://diamond.jp/articles/-/351526
[4] https://note.com/yuyadachi/n/n3ce209c497b3
[5] https://www.youtube.com/watch?v=NIOY4H4lBSQ
[6] https://nuco.co.jp/blog/article/LDKb_5Ez
```

Output:
```markdown
OpenAIは最近、新しい言語モデルの開発で注目を集めています。

2024年9月12日に「OpenAI o1」と「OpenAI o1-mini」という新しいモデルをリリースしました[(記事1)](https://weel.co.jp/media/innovator/openai-o1/) [(記事3)](https://diamond.jp/articles/-/351526) 。OpenAI o1は、特に理数系科目やコーディングにおいて高い性能を示し、複雑な問題解決能力が向上しています[(記事1)](https://weel.co.jp/media/innovator/openai-o1/) [(記事3)](https://diamond.jp/articles/-/351526) 。

OpenAI o1の主な特徴:
- 独自の「推論トークン」を使用し、複雑な問題に対応できる思考力を持つ[(記事1)](https://weel.co.jp/media/innovator/openai-o1/) 
- 理数科目やコーディングで専門家レベルの性能を発揮[(記事1)](https://weel.co.jp/media/innovator/openai-o1/) 
- プロンプトインジェクションへの耐性が向上[(記事1)](https://weel.co.jp/media/innovator/openai-o1/) 

OpenAI o1-miniは、理数系科目に特化した小型モデルで、処理速度が約3~5倍高速になり、コストも80%削減されています[(記事1)](https://weel.co.jp/media/innovator/openai-o1/) 。

さらに、2024年12月21日には「OpenAI o3」の初期バージョンが発表され、o1をさらに上回る性能を達成したと報告されています[(記事2)](https://gihyo.jp/article/2024/12/openai-o3-early-version) 。

これらの新モデルの登場により、OpenAIは生成AI市場でのリーダーシップを強化し、ユーザーの期待に応える革新的な技術を提供し続けているという評価を得ています[(記事3)](https://diamond.jp/articles/-/351526) 。

Citations:
```