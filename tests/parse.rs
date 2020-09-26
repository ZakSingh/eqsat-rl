use tamago::parse::*;

// parse_model takes a string which is the serialized model from taso
// here the string is the content of bts.model.
// taso.export_to_file from the taso python binding serializes the model

// so the end to end workflow is:

// 1. use the taso python binding to import the ONNX model
// 2. serialize the model with export_to_file
// 3. parse the output into a GraphConverter with model_parser

#[test]
fn model_parser() {
    parse_model(
        "100
0
10:0
64,1024
102
1
11:0
1024,1024
103
1
11:0
1024,1024
104
1
11:0
1024,1024
118
1
11:0
1024,1024
120
1
11:0
1024,1024
121
1
11:0
1024,1024
122
1
11:0
1024,1024
136
1
11:0
1024,1024
138
1
11:0
1024,1024
139
1
11:0
1024,1024
140
1
11:0
1024,1024
154
1
11:0
1024,1024
156
1
11:0
1024,1024
157
1
11:0
1024,1024
158
1
11:0
1024,1024
172
1
11:0
1024,1024
174
1
11:0
1024,1024
175
1
11:0
1024,1024
176
1
11:0
1024,1024
190
1
11:0
1024,1024
192
1
11:0
1024,1024
193
1
11:0
1024,1024
194
1
11:0
1024,1024
208
1
11:0
1024,1024
210
1
11:0
1024,1024
211
1
11:0
1024,1024
212
1
11:0
1024,1024
226
1
11:0
1024,1024
228
1
11:0
1024,1024
229
1
11:0
1024,1024
230
1
11:0
1024,1024
244
1
11:0
1024,1024
101
8
100:0
64,1024
105
18
101:0,102:0
0,2
106
18
101:0,103:0
0,2
107
18
101:0,104:0
0,2
119
18
101:0,118:0
0,2
108
14
105:0
64,16,64
109
14
106:0
64,16,64
110
14
107:0
64,16,64
123
18
119:0,120:0
0,2
124
18
119:0,121:0
0,2
125
18
119:0,122:0
0,2
137
18
119:0,136:0
0,2
111
15
108:0
1,0,2,1
112
15
109:0
1,0,2,1
113
15
110:0
1,0,2,1
126
14
123:0
64,16,64
127
14
124:0
64,16,64
128
14
125:0
64,16,64
141
18
137:0,138:0
0,2
142
18
137:0,139:0
0,2
143
18
137:0,140:0
0,2
155
18
137:0,154:0
0,2
114
18
111:0,112:0
0,3
129
15
126:0
1,0,2,1
130
15
127:0
1,0,2,1
131
15
128:0
1,0,2,1
144
14
141:0
64,16,64
145
14
142:0
64,16,64
146
14
143:0
64,16,64
159
18
155:0,156:0
0,2
160
18
155:0,157:0
0,2
161
18
155:0,158:0
0,2
173
18
155:0,172:0
0,2
115
18
113:0,114:0
0,3
132
18
129:0,130:0
0,3
147
15
144:0
1,0,2,1
148
15
145:0
1,0,2,1
149
15
146:0
1,0,2,1
162
14
159:0
64,16,64
163
14
160:0
64,16,64
164
14
161:0
64,16,64
177
18
173:0,174:0
0,2
178
18
173:0,175:0
0,2
179
18
173:0,176:0
0,2
191
18
173:0,190:0
0,2
116
15
115:0
1,0,2,1
133
18
131:0,132:0
0,3
150
18
147:0,148:0
0,3
165
15
162:0
1,0,2,1
166
15
163:0
1,0,2,1
167
15
164:0
1,0,2,1
180
14
177:0
64,16,64
181
14
178:0
64,16,64
182
14
179:0
64,16,64
195
18
191:0,192:0
0,2
196
18
191:0,193:0
0,2
197
18
191:0,194:0
0,2
209
18
191:0,208:0
0,2
117
14
116:0
64,1024
134
15
133:0
1,0,2,1
151
18
149:0,150:0
0,3
168
18
165:0,166:0
0,3
183
15
180:0
1,0,2,1
184
15
181:0
1,0,2,1
185
15
182:0
1,0,2,1
198
14
195:0
64,16,64
199
14
196:0
64,16,64
200
14
197:0
64,16,64
213
18
209:0,210:0
0,2
214
18
209:0,211:0
0,2
215
18
209:0,212:0
0,2
227
18
209:0,226:0
0,2
135
14
134:0
64,1024
152
15
151:0
1,0,2,1
169
18
167:0,168:0
0,3
186
18
183:0,184:0
0,3
201
15
198:0
1,0,2,1
202
15
199:0
1,0,2,1
203
15
200:0
1,0,2,1
216
14
213:0
64,16,64
217
14
214:0
64,16,64
218
14
215:0
64,16,64
231
18
227:0,228:0
0,2
232
18
227:0,229:0
0,2
233
18
227:0,230:0
0,2
245
18
227:0,244:0
0,2
153
14
152:0
64,1024
170
15
169:0
1,0,2,1
187
18
185:0,186:0
0,3
204
18
201:0,202:0
0,3
219
15
216:0
1,0,2,1
220
15
217:0
1,0,2,1
221
15
218:0
1,0,2,1
234
14
231:0
64,16,64
235
14
232:0
64,16,64
236
14
233:0
64,16,64
171
14
170:0
64,1024
188
15
187:0
1,0,2,1
205
18
203:0,204:0
0,3
222
18
219:0,220:0
0,3
237
15
234:0
1,0,2,1
238
15
235:0
1,0,2,1
239
15
236:0
1,0,2,1
189
14
188:0
64,1024
206
15
205:0
1,0,2,1
223
18
221:0,222:0
0,3
240
18
237:0,238:0
0,3
207
14
206:0
64,1024
224
15
223:0
1,0,2,1
241
18
239:0,240:0
0,3
225
14
224:0
64,1024
242
15
241:0
1,0,2,1
243
14
242:0
64,1024",
    );
}
