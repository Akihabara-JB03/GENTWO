import strutils
type
    TokenType* = enum
        GT_SET,
        GT_TO,
        GT_PERIOD,
        GT_NUL,
        GT_IDENT,
        GT_INT,
        GT_COMPARE,
        GT_EQ,
        GT_EXIT,
        GT_FLOAT,
        GT_STRING,
        GT_PLUS,
        GT_MINUS,
        GT_DIVISION,
        GT_TIMES,
        GT_INDENT,
        GT_DEDENT,
        GT_DISPLAYS,
        GT_NEQ,
        GT_COMMENT,
        GT_LOAD,
        GT_LP, #Left Paren
        GT_RP, #Right Paren
        GT_DEF,
        GT_FUNC,
        GT_OVER,
        GT_UNDER
    Token* = object
        kind*: TokenType
        literal*: string
proc tokenize*(source: string): seq[Token] =
    var resultTokens: seq[Token] = @[]
    let lines = source.splitLines()
    var previousIndent = 0
    for line in lines:
        var indent = 0
        for ch in line:
            if ch == '\t':
                indent += 1
            else:
                break
        if indent > previousIndent:
            for i in 0 ..< indent - previousIndent:
                resultTokens.add(Token(kind: GT_INDENT, literal: ""))
        elif indent < previousIndent:
            for j in 0 ..< previousIndent - indent:
                resultTokens.add(Token(kind: GT_DEDENT, literal: ""))
        previousIndent = indent
        let words = line.splitWhitespace()
        for word in words:
            if word.startsWith("\"") or word.endsWith("\""):
                resultTokens.add(Token(kind:GT_STRING,literal:word.strip(chars = {'"'})))
            elif word[0] in {'0'..'9'}:
                if '.' in word:
                    try:
                        discard parseFloat(word)
                        resultTokens.add(Token(kind: GT_FLOAT, literal: word))
                    except ValueError:
                        resultTokens.add(Token(kind: GT_IDENT, literal: word))
                else:
                    resultTokens.add(Token(kind: GT_INT, literal: word))
            else:
                case word
                    of "SET":
                        resultTokens.add(Token(kind:GT_SET,literal:"SET"))
                    of "FLOAT":
                        resultTokens.add(Token(kind:GT_FLOAT,literal:"FLOAT"))
                    of "INT":
                        resultTokens.add(Token(kind:GT_INT,literal:"INT"))
                    of "TO":
                        resultTokens.add(Token(kind:GT_TO,literal:"TO"))
                    of "COMPARE":
                        resultTokens.add(Token(kind:GT_COMPARE,literal:"COMPARE"))
                    of ".":
                        resultTokens.add(Token(kind:GT_PERIOD,literal:"."))
                    of "EXIT":
                        resultTokens.add(Token(kind:GT_EXIT,literal:"EXIT"))
                    of "DISPLAYS":
                        resultTokens.add(Token(kind:GT_DISPLAYS,literal:"DISPLAYS"))
                    of "PLUS":
                        resultTokens.add(Token(kind:GT_PLUS,literal:"PLUS"))
                    of "MINUS":
                        resultTokens.add(Token(kind:GT_MINUS,literal:"MINUS"))
                    of "TIMES":
                        resultTokens.add(Token(kind:GT_TIMES,literal:"TIMES"))
                    of "DIV":
                        resultTokens.add(Token(kind:GT_DIVISION,literal:"DIV"))
                    of "COMMENT":
                        resultTokens.add(Token(kind:GT_COMMENT,literal:"COMM"))
                    of "NULL":
                        resultTokens.add(Token(kind:GT_NUL,literal:"NUL"))
                    of "Load":
                        resultTokens.add(Token(kind:GT_LOAD,literal:"Load"))
                    of "DEFINE":
                        resultTokens.add(Token(kind:GT_DEF,literal:"DEFINE"))
                    of "FUNC":
                        resultTokens.add(Token(kind:GT_FUNC,literal:"FUNC"))
                    of "EQUAL":
                        resultTokens.add(Token(kind:GT_EQ,literal:"EQUAL"))
                    of "NOTEQUAL":
                        resultTokens.add(Token(kind:GT_NEQ,literal:"NOTEQUAL"))
                    else:
                        resultTokens.add(Token(kind:GT_IDENT,literal:word))
    return resultTokens